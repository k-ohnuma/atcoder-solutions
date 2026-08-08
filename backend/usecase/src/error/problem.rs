use domain::error::{
    external::ExternalError, problem::ProblemValidationError, repository::RepositoryError,
};
use thiserror::Error;

#[derive(thiserror::Error, Debug)]
pub enum ImportProblemsUsecaseError {
    #[error(transparent)]
    Fetch(#[from] ExternalError),
    #[error(transparent)]
    Repository(#[from] RepositoryError),
}

#[derive(Debug, Error)]
pub enum ProblemError {
    #[error("{0}")]
    BadRequest(String),
    #[error("{0}")]
    Internal(String),
    #[error("{0}")]
    Unavailable(String),
    #[error("{0}")]
    NotFound(String),
}

impl From<RepositoryError> for ProblemError {
    fn from(value: RepositoryError) -> Self {
        match value {
            RepositoryError::Unavailable(msg) => ProblemError::Unavailable(msg),
            RepositoryError::Unexpected(msg) => ProblemError::Internal(msg),
        }
    }
}

impl From<ProblemValidationError> for ProblemError {
    fn from(value: ProblemValidationError) -> Self {
        ProblemError::BadRequest(value.to_string())
    }
}
