use domain::error::repository::RepositoryError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SolutionError {
    #[error("{0}")]
    BadRequest(String),
    #[error("{0}")]
    Forbidden(String),
    #[error("{0}")]
    Internal(String),
    #[error("{0}")]
    NotFound(String),
    #[error("{0}")]
    Conflict(String),
}

impl From<RepositoryError> for SolutionError {
    fn from(value: RepositoryError) -> Self {
        match value {
            RepositoryError::NotFound(msg) => SolutionError::NotFound(msg),
            RepositoryError::TransactionError(msg) => SolutionError::Internal(msg.to_string()),
            RepositoryError::UniqueViolation(msg) => SolutionError::Conflict(msg),
            RepositoryError::ForeignKeyViolation(msg) => SolutionError::Internal(msg.to_string()),
            RepositoryError::NotNullViolation(msg) => SolutionError::Internal(msg.to_string()),
            RepositoryError::CheckViolation(msg) => SolutionError::Internal(msg.to_string()),
            RepositoryError::Connection(msg) => SolutionError::Internal(msg.to_string()),
            RepositoryError::Query(msg) => SolutionError::Internal(msg.to_string()),
            RepositoryError::Unexpected(msg) => SolutionError::Internal(msg.to_string()),
        }
    }
}
