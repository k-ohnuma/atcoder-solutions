use shared::error::{http::HttpError, repository::RepositoryError};
use thiserror::Error;

pub mod create;
pub mod get_contest_group_by_contest_series;
pub mod get_by_contest;

#[derive(Debug, Error)]
pub enum ProblemError {
    #[error("{0}")]
    BadRequest(String),
    #[error("{0}")]
    DBError(String),
    #[error("{0}")]
    NotFound(String),
}

impl From<RepositoryError> for ProblemError {
    fn from(value: RepositoryError) -> Self {
        match value {
            RepositoryError::NotFound(msg) => ProblemError::NotFound(msg),
            RepositoryError::TransactionError(msg) => ProblemError::DBError(msg.to_string()),
            RepositoryError::UniqueViolation(msg) => ProblemError::DBError(msg),
            RepositoryError::ForeignKeyViolation(msg) => ProblemError::DBError(msg.to_string()),
            RepositoryError::NotNullViolation(msg) => ProblemError::DBError(msg.to_string()),
            RepositoryError::CheckViolation(msg) => ProblemError::DBError(msg.to_string()),
            RepositoryError::Connection(msg) => ProblemError::DBError(msg.to_string()),
            RepositoryError::Query(msg) => ProblemError::DBError(msg.to_string()),
            RepositoryError::Unexpected(msg) => ProblemError::DBError(msg.to_string()),
        }
    }
}

impl From<ProblemError> for HttpError {
    fn from(value: ProblemError) -> Self {
        match value {
            ProblemError::BadRequest(reason) => HttpError::BadRequest(reason),
            ProblemError::NotFound(reason) => HttpError::NotFound(reason),
            ProblemError::DBError(reason) => HttpError::Internal(reason),
        }
    }
}
