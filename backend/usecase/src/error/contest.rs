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
            RepositoryError::TransactionError(msg) => ContestError::Internal(msg.to_string()),
            RepositoryError::UniqueViolation(msg) => ContestError::Internal(msg),
            RepositoryError::ForeignKeyViolation(msg) => ContestError::Internal(msg.to_string()),
            RepositoryError::NotNullViolation(msg) => ContestError::Internal(msg.to_string()),
            RepositoryError::CheckViolation(msg) => ContestError::Internal(msg.to_string()),
            RepositoryError::Connection(msg) => ContestError::Internal(msg.to_string()),
            RepositoryError::Query(msg) => ContestError::Internal(msg.to_string()),
            RepositoryError::Unexpected(msg) => ContestError::Internal(msg.to_string()),
        }
    }
}
