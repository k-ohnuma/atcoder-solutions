use domain::{
    error::{
        repository::RepositoryError,
        user::{CreateUserRepositoryError, UserValidationError},
    },
    ports::external::auth::AuthError,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("{0}")]
    BadRequest(String),
    #[error("{0}")]
    Internal(String),
    #[error("{0}")]
    NotFound(String),
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
            RepositoryError::Unavailable(msg) | RepositoryError::Unexpected(msg) => {
                UserError::Internal(msg)
            }
        }
    }
}

impl From<CreateUserRepositoryError> for UserError {
    fn from(value: CreateUserRepositoryError) -> Self {
        match value {
            CreateUserRepositoryError::UserNameAlreadyExists => {
                UserError::UserNameAlreadyExists("user name already exists".to_string())
            }
            CreateUserRepositoryError::Repository(error) => UserError::from(error),
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
                UserError::Internal("identity provider temporarily unavailable".to_string())
            }
            AuthError::Unexpected(msg) => UserError::Internal(msg),
        }
    }
}

impl From<UserValidationError> for UserError {
    fn from(value: UserValidationError) -> Self {
        UserError::BadRequest(value.to_string())
    }
}
