use reqwest::StatusCode;
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
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl From<reqwest::Error> for ExternalError {
    fn from(err: reqwest::Error) -> Self {
        let url = err.url();
        tracing::error!(error=?err, url=?url, "external request failed");

        if err.is_timeout() || err.is_connect() {
            return ExternalError::ExternalUnavailable;
        }
        if let Some(s) = err.status() {
            return match s {
                StatusCode::NOT_FOUND => ExternalError::NotFound,
                StatusCode::UNAUTHORIZED => ExternalError::Unauthorized,
                StatusCode::FORBIDDEN => ExternalError::Forbidden,
                StatusCode::TOO_MANY_REQUESTS => ExternalError::RateLimited,
                s if s.is_server_error() => ExternalError::ExternalUnavailable,
                _ => ExternalError::Other(err.into()),
            };
        }
        ExternalError::Other(err.into())
    }
}

impl From<serde_json::Error> for ExternalError {
    fn from(err: serde_json::Error) -> Self {
        tracing::error!(error = ?err, "invalid json format");
        ExternalError::InvalidJson(err.to_string())
    }
}
