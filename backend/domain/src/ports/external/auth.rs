use async_trait::async_trait;
use thiserror::Error;

pub struct Principal {
    pub uid: String,
    pub email: String,
}

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("unauthorized")]
    Unauthorized,
    #[error("temporarily unavailable")]
    TemporarilyUnavailable,
}

#[async_trait]
pub trait AuthenticatorPort: Send + Sync {
    async fn verify_id_token(&self, token: &str) -> Result<Principal, AuthError>;
}
