use async_trait::async_trait;
use thiserror::Error;

pub struct Principal {
    pub uid: String,
    pub email: String,
    pub issued_at: i64,
}

pub struct CreatedIdentity {
    pub uid: String,
}

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("unauthorized")]
    Unauthorized,
    #[error("{0}")]
    BadRequest(String),
    #[error("{0}")]
    Conflict(String),
    #[error("temporarily unavailable")]
    TemporarilyUnavailable,
    #[error("{0}")]
    Unexpected(String),
}

#[async_trait]
pub trait AuthenticatorPort: Send + Sync {
    async fn verify_id_token(&self, token: &str) -> Result<Principal, AuthError>;
}

#[async_trait]
pub trait IdentityProviderPort: Send + Sync {
    async fn create_user(&self, email: &str, password: &str) -> Result<CreatedIdentity, AuthError>;
    async fn delete_user(&self, uid: &str) -> Result<(), AuthError>;
}
