use domain::{
    error::{
        repository::RepositoryError,
        user::{CreateUserRepositoryError, UserValidationError},
    },
    ports::external::auth::{AuthProviderError, CreateIdentityError},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("{0}")]
    BadRequest(String),
    #[error("{0}")]
    Internal(String),
    #[error("{0}")]
    Unavailable(String),
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
            RepositoryError::Unavailable(msg) => UserError::Unavailable(msg),
            RepositoryError::Unexpected(msg) => UserError::Internal(msg),
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

impl From<AuthProviderError> for UserError {
    fn from(value: AuthProviderError) -> Self {
        match value {
            AuthProviderError::TemporarilyUnavailable => {
                UserError::Unavailable("identity provider temporarily unavailable".to_string())
            }
            AuthProviderError::Unexpected(msg) => UserError::Internal(msg),
        }
    }
}

impl From<CreateIdentityError> for UserError {
    fn from(value: CreateIdentityError) -> Self {
        match value {
            CreateIdentityError::EmailAlreadyExists => {
                UserError::EmailAlreadyExists("email already exists".to_string())
            }
            CreateIdentityError::InvalidEmail => UserError::BadRequest("invalid email".to_string()),
            CreateIdentityError::WeakPassword => UserError::BadRequest("weak password".to_string()),
            CreateIdentityError::Provider(error) => UserError::from(error),
        }
    }
}

impl From<UserValidationError> for UserError {
    fn from(value: UserValidationError) -> Self {
        UserError::BadRequest(value.to_string())
    }
}
