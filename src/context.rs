#[derive(Clone)]
pub struct Context {
    pub auth_token: Option<String>,
}

impl Context {
    pub fn new() -> Self {
        Self { auth_token: None }
    }

    pub fn is_authenticated(&self) -> bool {
        self.auth_token.is_some()
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}
