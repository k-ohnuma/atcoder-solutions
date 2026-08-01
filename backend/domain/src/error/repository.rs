use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("repository resource not found: {0}")]
    NotFound(String),

    #[error("repository conflict: {0}")]
    Conflict(String),

    #[error("repository unavailable: {0}")]
    Unavailable(String),

    #[error("unexpected repository error: {0}")]
    Unexpected(String),
}
