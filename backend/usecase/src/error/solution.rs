use domain::error::{
    problem::ProblemValidationError, repository::RepositoryError,
    solution::SolutionValidationError, user::UserValidationError,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SolutionError {
    #[error("{0}")]
    BadRequest(String),
    #[error("{0}")]
    Forbidden(String),
    #[error("{0}")]
    Internal(String),
    #[error("{0}")]
    NotFound(String),
}

impl From<RepositoryError> for SolutionError {
    fn from(value: RepositoryError) -> Self {
        match value {
            RepositoryError::Unavailable(msg) | RepositoryError::Unexpected(msg) => {
                SolutionError::Internal(msg)
            }
        }
    }
}

impl From<SolutionValidationError> for SolutionError {
    fn from(value: SolutionValidationError) -> Self {
        SolutionError::BadRequest(value.to_string())
    }
}

impl From<ProblemValidationError> for SolutionError {
    fn from(value: ProblemValidationError) -> Self {
        SolutionError::BadRequest(value.to_string())
    }
}

impl From<UserValidationError> for SolutionError {
    fn from(value: UserValidationError) -> Self {
        SolutionError::BadRequest(value.to_string())
    }
}
