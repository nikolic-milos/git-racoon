use crate::auth::auth_error::AuthError;
use keyring::Entry;
use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;
use tokio::time::Duration;

const GITHUB_CLIENT_ID: &str = "Ov23liRkELhLXqS82Irs";
const SERVICE_NAME: &str = "GitRaccoon";
const USERNAME: &str = "github_token";

#[derive(serde::Serialize)]
struct DeviceCodeRequest {
    client_id: &'static str,
    scope: &'static str,
}

#[derive(Deserialize)]
pub struct DeviceCodeResponse {
    device_code: String,
    user_code: String,
    verification_uri: String,
    expires_in: u64,
    interval: u64,
}

#[derive(serde::Serialize)]
struct TokenRequest<'a> {
    client_id: &'static str,
    device_code: &'a str,
    grant_type: &'static str,
}

#[derive(Deserialize)]
pub struct TokenResponse {
    access_token: String,
    token_type: String,
    scope: String,
}

async fn request_device_code() -> Result<(String, String, u64), AuthError> {
    let client = Client::new();

    let response = client
        .post("https://github.com/login/device/code")
        .header("Accept", "application/json")
        .json(&DeviceCodeRequest {
            client_id: GITHUB_CLIENT_ID,
            scope: "repo",
        })
        .send()
        .await
        .map_err(|e| AuthError::RequestFailed(e.to_string()))?;

    if !response.status().is_success() {
        return Err(AuthError::RequestFailed(format!(
            "GitHub returned {}",
            response.status()
        )));
    }

    let resp: DeviceCodeResponse = response
        .json()
        .await
        .map_err(|e| AuthError::ParseFailed(e.to_string()))?;

    Ok((resp.device_code, resp.user_code, resp.interval))
}

pub async fn poll_for_token(device_code: &str, interval: u64) -> Result<String, AuthError> {
    let client = Client::new();

    loop {
        let response = client
            .post("https://github.com/login/oauth/access_token")
            .header("Accept", "application/json")
            .json(&TokenRequest {
                client_id: GITHUB_CLIENT_ID,
                device_code,
                grant_type: "urn:ietf:params:oauth:grant-type:device_code",
            })
            .send()
            .await
            .map_err(|e| AuthError::RequestFailed(e.to_string()))?;

        if response.status().is_success() {
            let token_resp: TokenResponse = response
                .json()
                .await
                .map_err(|e| AuthError::ParseFailed(e.to_string()))?;

            return Ok(token_resp.access_token);
        }

        let error_resp: Value = response
            .json()
            .await
            .unwrap_or(Value::Object(Default::default()));

        let error = error_resp["error"].as_str().unwrap_or("unknown");

        match error {
            "authorization_pending" => {
                tokio::time::sleep(Duration::from_secs(interval)).await;
            }

            "slow_down" => {
                tokio::time::sleep(Duration::from_secs(interval + 5)).await;
            }

            "expired_token" | "access_denied" => {
                return Err(AuthError::AuthFailed(error.to_string()));
            }

            _ => {
                tokio::time::sleep(Duration::from_secs(interval)).await;
            }
        }
    }
}

pub async fn authenticate_github() -> Result<String, AuthError> {
    let (device_code, user_code, interval) = request_device_code().await?;

    println!("Open github.com/device and enter code: {}", user_code);
    println!("Waiting for authentication...");

    let access_token = poll_for_token(&device_code, interval).await?;

    store_token(&access_token)?;

    Ok(access_token)
}

pub fn store_token(token: &str) -> Result<(), AuthError> {
    let entry =
        Entry::new(SERVICE_NAME, USERNAME).map_err(|e| AuthError::KeyringFailed(e.to_string()))?;

    entry
        .set_password(token)
        .map_err(|e| AuthError::KeyringFailed(e.to_string()))?;
    Ok(())
}

pub fn load_token() -> Result<Option<String>, AuthError> {
    let entry =
        Entry::new(SERVICE_NAME, USERNAME).map_err(|e| AuthError::KeyringFailed(e.to_string()))?;

    match entry.get_password() {
        Ok(token) => Ok(Some(token)),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(e) => Err(AuthError::KeyringFailed(e.to_string()))?,
    }
}

pub fn delete_token() -> Result<(), AuthError> {
    let entry =
        Entry::new(SERVICE_NAME, USERNAME).map_err(|e| AuthError::KeyringFailed(e.to_string()))?;

    entry
        .delete_credential()
        .map_err(|e| AuthError::KeyringFailed(e.to_string()))?;
    Ok(())
}
