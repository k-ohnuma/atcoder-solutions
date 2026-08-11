use thiserror::Error;

#[derive(Debug, Error)]
pub enum AtcoderProblemsError {
    #[error("AtCoder problems service is temporarily unavailable")]
    TemporarilyUnavailable,

    #[error("invalid AtCoder problems response: {0}")]
    InvalidResponse(String),

    #[error("unexpected AtCoder problems error: {0}")]
    Unexpected(String),
}
