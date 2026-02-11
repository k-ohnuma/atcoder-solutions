use domain::error::{external::ExternalError, repository::RepositoryError};

#[derive(thiserror::Error, Debug)]
pub enum ImportProblemsUsecaseError {
    #[error(transparent)]
    Fetch(#[from] ExternalError),
    #[error(transparent)]
    Repository(#[from] RepositoryError),
}
