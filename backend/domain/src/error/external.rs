use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExternalError {
    #[error("not found")]
    NotFound,
    #[error("unauthorized")]
    Unauthorized,
    #[error("forbidden")]
    Forbidden,
    #[error("rate limited")]
    RateLimited,
    #[error("temporarily unavailable")]
    ExternalUnavailable,
    #[error("invalid json: {0}")]
    InvalidJson(String),
    #[error("other external error: {0}")]
    Other(String),
}
