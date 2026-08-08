use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("repository unavailable: {0}")]
    Unavailable(String),

    #[error("unexpected repository error: {0}")]
    Unexpected(String),
}
