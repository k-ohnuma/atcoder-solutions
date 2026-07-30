use domain::error::{external::ExternalError, repository::RepositoryError};
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
    DBError(String),
    #[error("{0}")]
    NotFound(String),
}

impl From<RepositoryError> for ProblemError {
    fn from(value: RepositoryError) -> Self {
        match value {
            RepositoryError::NotFound(msg) => ProblemError::NotFound(msg),
            RepositoryError::TransactionError(msg) => ProblemError::DBError(msg.to_string()),
            RepositoryError::UniqueViolation(msg) => ProblemError::DBError(msg),
            RepositoryError::ForeignKeyViolation(msg) => ProblemError::DBError(msg.to_string()),
            RepositoryError::NotNullViolation(msg) => ProblemError::DBError(msg.to_string()),
            RepositoryError::CheckViolation(msg) => ProblemError::DBError(msg.to_string()),
            RepositoryError::Connection(msg) => ProblemError::DBError(msg.to_string()),
            RepositoryError::Query(msg) => ProblemError::DBError(msg.to_string()),
            RepositoryError::Unexpected(msg) => ProblemError::DBError(msg.to_string()),
        }
    }
}
