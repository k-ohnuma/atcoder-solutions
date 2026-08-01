use domain::error::repository::RepositoryError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ContestError {
    #[error("{0}")]
    BadRequest(String),
    #[error("{0}")]
    Internal(String),
    #[error("{0}")]
    NotFound(String),
}

impl From<RepositoryError> for ContestError {
    fn from(value: RepositoryError) -> Self {
        match value {
            RepositoryError::NotFound(msg) => ContestError::NotFound(msg),
            RepositoryError::Conflict(msg)
            | RepositoryError::Unavailable(msg)
            | RepositoryError::Unexpected(msg) => ContestError::Internal(msg),
        }
    }
}
