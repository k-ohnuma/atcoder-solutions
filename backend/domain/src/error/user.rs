use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum UserValidationError {
    #[error("userId cannot be empty")]
    BlankId,
}
