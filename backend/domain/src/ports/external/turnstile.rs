use async_trait::async_trait;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TurnstileError {
    #[error("turnstile token is required")]
    TokenRequired,
    #[error("turnstile token is invalid")]
    InvalidToken,
    #[error("turnstile temporarily unavailable")]
    TemporarilyUnavailable,
    #[error("{0}")]
    Unexpected(String),
}

#[async_trait]
pub trait TurnstileVerifierPort: Send + Sync {
    async fn verify(&self, token: Option<&str>) -> Result<(), TurnstileError>;
}
