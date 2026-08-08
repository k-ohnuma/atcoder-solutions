use domain::error::repository::RepositoryError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ContestError {
    #[error("{0}")]
    BadRequest(String),
    #[error("{0}")]
    Internal(String),
}

impl From<RepositoryError> for ContestError {
    fn from(value: RepositoryError) -> Self {
        match value {
            RepositoryError::Unavailable(msg) | RepositoryError::Unexpected(msg) => {
                ContestError::Internal(msg)
            }
        }
    }
}
