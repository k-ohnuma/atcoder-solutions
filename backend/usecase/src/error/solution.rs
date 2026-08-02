use domain::error::{
    problem::ProblemValidationError, repository::RepositoryError, solution::SolutionValidationError,
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
    #[error("{0}")]
    Conflict(String),
}

impl From<RepositoryError> for SolutionError {
    fn from(value: RepositoryError) -> Self {
        match value {
            RepositoryError::NotFound(msg) => SolutionError::NotFound(msg),
            RepositoryError::Conflict(msg) => SolutionError::Conflict(msg),
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
