#[derive(Debug)]
pub enum AuthError {
    RequestFailed(String),
    ParseFailed(String),
    AuthFailed(String),
    KeyringFailed(String),
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::RequestFailed(msg) => write!(f, "Request failed: {}", msg),
            AuthError::ParseFailed(msg) => write!(f, "Parse failed: {}", msg),
            AuthError::AuthFailed(msg) => write!(f, "Authentication failed: {}", msg),
            AuthError::KeyringFailed(msg) => write!(f, "Keyring failed: {}", msg),
        }
    }
}

impl std::error::Error for AuthError {}
