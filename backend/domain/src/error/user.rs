use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum UserValidationError {
    #[error("userId cannot be empty")]
    BlankId,
    #[error("userName cannot be empty")]
    BlankName,
    #[error("userName must be {max} characters or fewer")]
    NameTooLong { max: usize },
    #[error("userName must contain only ASCII letters, numbers, or underscores")]
    InvalidNameFormat,
}
