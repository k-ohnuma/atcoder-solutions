use shared::error::{http::HttpError, repository::RepositoryError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ContestError {
    #[error("{0}")]
    BadRequest(String),
    #[error("{0}")]
    DBError(String),
    #[error("{0}")]
    NotFound(String),
}

impl From<RepositoryError> for ContestError {
    fn from(value: RepositoryError) -> Self {
        match value {
            RepositoryError::NotFound(msg) => ContestError::NotFound(msg),
            RepositoryError::TransactionError(msg) => ContestError::DBError(msg.to_string()),
            RepositoryError::UniqueViolation(msg) => ContestError::DBError(msg),
            RepositoryError::ForeignKeyViolation(msg) => ContestError::DBError(msg.to_string()),
            RepositoryError::NotNullViolation(msg) => ContestError::DBError(msg.to_string()),
            RepositoryError::CheckViolation(msg) => ContestError::DBError(msg.to_string()),
            RepositoryError::Connection(msg) => ContestError::DBError(msg.to_string()),
            RepositoryError::Query(msg) => ContestError::DBError(msg.to_string()),
            RepositoryError::Unexpected(msg) => ContestError::DBError(msg.to_string()),
        }
    }
}

impl From<ContestError> for HttpError {
    fn from(value: ContestError) -> Self {
        match value {
            ContestError::BadRequest(reason) => HttpError::BadRequest(reason),
            ContestError::NotFound(reason) => HttpError::NotFound(reason),
            ContestError::DBError(reason) => HttpError::Internal(reason),
        }
    }
}
