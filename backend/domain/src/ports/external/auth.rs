use async_trait::async_trait;
use thiserror::Error;

use crate::model::user::id::UserId;

pub struct Principal {
    pub uid: UserId,
    pub email: String,
    pub issued_at: i64,
    pub auth_time: i64,
}

pub struct CreatedIdentity {
    pub uid: UserId,
}

#[derive(Debug, Error)]
pub enum AuthProviderError {
    #[error("temporarily unavailable")]
    TemporarilyUnavailable,
    #[error("{0}")]
    Unexpected(String),
}

#[derive(Debug, Error)]
pub enum VerifyTokenError {
    #[error("invalid token")]
    InvalidToken,
    #[error(transparent)]
    Provider(#[from] AuthProviderError),
}

#[derive(Debug, Error)]
pub enum CreateIdentityError {
    #[error("email already exists")]
    EmailAlreadyExists,
    #[error("invalid email")]
    InvalidEmail,
    #[error("weak password")]
    WeakPassword,
    #[error(transparent)]
    Provider(#[from] AuthProviderError),
}

#[async_trait]
pub trait AuthenticatorPort: Send + Sync {
    async fn verify_id_token(&self, token: &str) -> Result<Principal, VerifyTokenError>;
}

#[async_trait]
pub trait IdentityProviderPort: Send + Sync {
    async fn create_user(
        &self,
        email: &str,
        password: &str,
    ) -> Result<CreatedIdentity, CreateIdentityError>;
    async fn delete_user(&self, uid: &UserId) -> Result<(), AuthProviderError>;
}
