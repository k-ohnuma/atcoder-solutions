pub mod create;
pub mod delete_me;
pub mod get_me;
pub mod revoke_tokens;

use domain::error::repository::RepositoryError;
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

impl From<RepositoryError> for UserError {
    fn from(value: RepositoryError) -> Self {
        match value {
            RepositoryError::NotFound(msg) => UserError::NotFound(msg),
            RepositoryError::TransactionError(msg) => UserError::DBError(msg.to_string()),
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
