use domain::error::repository::RepositoryError;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct ContestListItem {
    pub code: String,
    pub series_code: String,
}

#[derive(Debug, Error)]
pub enum ContestError {
    #[error("{0}")]
    BadRequest(String),
    #[error("{0}")]
    DBError(String),
    #[error("{0}")]
    NotFound(String),
}

impl From<RepositoryError> for ContestError {
    fn from(value: RepositoryError) -> Self {
        match value {
            RepositoryError::NotFound(msg) => ContestError::NotFound(msg),
            RepositoryError::TransactionError(msg) => ContestError::DBError(msg.to_string()),
            RepositoryError::UniqueViolation(msg) => ContestError::DBError(msg),
            RepositoryError::ForeignKeyViolation(msg) => ContestError::DBError(msg.to_string()),
            RepositoryError::NotNullViolation(msg) => ContestError::DBError(msg.to_string()),
            RepositoryError::CheckViolation(msg) => ContestError::DBError(msg.to_string()),
            RepositoryError::Connection(msg) => ContestError::DBError(msg.to_string()),
            RepositoryError::Query(msg) => ContestError::DBError(msg.to_string()),
            RepositoryError::Unexpected(msg) => ContestError::DBError(msg.to_string()),
        }
    }
}
