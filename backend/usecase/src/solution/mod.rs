pub mod create;

use shared::error::{http::HttpError, repository::RepositoryError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SolutionError {
    #[error("{0}")]
    BadRequest(String),
    #[error("{0}")]
    DBError(String),
    #[error("{0}")]
    NotFound(String),
    #[error("{0}")]
    Conflict(String),
}

impl From<SolutionError> for HttpError {
    fn from(value: SolutionError) -> Self {
        match value {
            SolutionError::BadRequest(reason) => HttpError::BadRequest(reason),
            SolutionError::NotFound(reason) => HttpError::NotFound(reason),
            SolutionError::Conflict(reason) => HttpError::Conflict(reason),
            SolutionError::DBError(reason) => HttpError::Internal(reason),
        }
    }
}

impl From<RepositoryError> for SolutionError {
    fn from(value: RepositoryError) -> Self {
        match value {
            RepositoryError::NotFound(msg) => SolutionError::NotFound(msg),
            RepositoryError::TransactionError(msg) => SolutionError::DBError(msg.to_string()),
            RepositoryError::UniqueViolation(msg) => SolutionError::Conflict(msg),
            RepositoryError::ForeignKeyViolation(msg) => SolutionError::DBError(msg.to_string()),
            RepositoryError::NotNullViolation(msg) => SolutionError::DBError(msg.to_string()),
            RepositoryError::CheckViolation(msg) => SolutionError::DBError(msg.to_string()),
            RepositoryError::Connection(msg) => SolutionError::DBError(msg.to_string()),
            RepositoryError::Query(msg) => SolutionError::DBError(msg.to_string()),
            RepositoryError::Unexpected(msg) => SolutionError::DBError(msg.to_string()),
        }
    }
}
