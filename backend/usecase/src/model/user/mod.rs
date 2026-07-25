pub mod create;
pub mod delete_me;
pub mod get_me;
pub mod revoke_tokens;

use domain::error::repository::RepositoryError;
use domain::ports::external::auth::AuthError;
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
    #[error("{0}")]
    EmailAlreadyExists(String),
    #[error("{0}")]
    UserNameAlreadyExists(String),
    #[error("{0}")]
    RequiresRecentLogin(String),
}

impl From<RepositoryError> for UserError {
    fn from(value: RepositoryError) -> Self {
        match value {
            RepositoryError::NotFound(msg) => UserError::NotFound(msg),
            RepositoryError::TransactionError(msg) => UserError::DBError(msg.to_string()),
            RepositoryError::UniqueViolation(msg) => UserError::UserNameAlreadyExists(msg),
            RepositoryError::ForeignKeyViolation(msg) => UserError::DBError(msg.to_string()),
            RepositoryError::NotNullViolation(msg) => UserError::DBError(msg.to_string()),
            RepositoryError::CheckViolation(msg) => UserError::DBError(msg.to_string()),
            RepositoryError::Connection(msg) => UserError::DBError(msg.to_string()),
            RepositoryError::Query(msg) => UserError::DBError(msg.to_string()),
            RepositoryError::Unexpected(msg) => UserError::DBError(msg.to_string()),
        }
    }
}

impl From<AuthError> for UserError {
    fn from(value: AuthError) -> Self {
        match value {
            AuthError::Unauthorized => UserError::BadRequest("unauthorized".to_string()),
            AuthError::BadRequest(msg) => UserError::BadRequest(msg),
            AuthError::Conflict(msg) => UserError::EmailAlreadyExists(msg),
            AuthError::TemporarilyUnavailable => {
                UserError::DBError("identity provider temporarily unavailable".to_string())
            }
            AuthError::Unexpected(msg) => UserError::DBError(msg),
        }
    }
}
