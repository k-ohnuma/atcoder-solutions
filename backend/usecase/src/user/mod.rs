pub mod create_user;

use shared::error::{http::HttpError, repository::RepositoryError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("{0}")]
    BadRequest(String),
    #[error("{0}")]
    DBError(String),
    #[error("{0}")]
    NotFound(String),
    #[error("{0}")]
    Conflict(String),
}

impl From<UserError> for HttpError {
    fn from(value: UserError) -> Self {
        match value {
            UserError::BadRequest(reason) => HttpError::BadRequest(reason),
            UserError::NotFound(reason) => HttpError::NotFound(reason),
            UserError::Conflict(reason) => HttpError::Conflict(reason),
            UserError::DBError(reason) => HttpError::Internal(reason),
        }
    }
}

impl From<RepositoryError> for UserError {
    fn from(value: RepositoryError) -> Self {
        match value {
            RepositoryError::TransactionError(msg) => UserError::DBError(msg.to_string()),
            RepositoryError::NotFound(msg) => UserError::NotFound(msg),
            RepositoryError::UniqueViolation(msg) => UserError::Conflict(msg),
            RepositoryError::ForeignKeyViolation(msg) => UserError::DBError(msg.to_string()),
            RepositoryError::NotNullViolation(msg) => UserError::DBError(msg.to_string()),
            RepositoryError::CheckViolation(msg) => UserError::DBError(msg.to_string()),
            RepositoryError::Connection(msg) => UserError::DBError(msg.to_string()),
            RepositoryError::Query(msg) => UserError::DBError(msg.to_string()),
            RepositoryError::Unexpected(msg) => UserError::DBError(msg.to_string()),
        }
    }
}
