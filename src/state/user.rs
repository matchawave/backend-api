use reqwest::StatusCode;
use tracing::warn;

#[derive(Debug, Clone)]
pub enum RequestedUser {
    User,
    Bot(Bot),
    UserWithToken(User),
}

impl RequestedUser {
    pub fn is_bot(&self) -> bool {
        matches!(self, RequestedUser::Bot(_))
    }
    pub fn is_user(&self) -> bool {
        matches!(self, RequestedUser::User)
    }

    pub fn bot_protection(&self, status: &str) -> Result<(), (StatusCode, String)> {
        if !self.is_bot() {
            warn!("Non-bot user attempted to access {} endpoint", status);
            return Err((
                StatusCode::FORBIDDEN,
                format!("{} is only available to registered bots", status),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct User {
    access_token: String,
}

#[derive(Debug, Clone)]
pub struct Bot {
    token: String,
}

impl User {
    pub fn new(access_token: String) -> Self {
        Self { access_token }
    }
    pub fn access_token(&self) -> &str {
        &self.access_token
    }
}

impl Bot {
    pub fn new(token: String) -> Self {
        Self { token }
    }
    pub fn token(&self) -> &str {
        &self.token
    }
}
