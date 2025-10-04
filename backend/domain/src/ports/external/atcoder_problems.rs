use async_trait::async_trait;
use thiserror::Error;

use crate::model::atcoder_problems::ApiProblem;

#[derive(Error, Debug)]
pub enum AtcoderProblemsError {
    #[error("problems json not found")]
    NotFound,
    #[error("external temporarily unavailable")]
    ExternalUnavailable,
    #[error("invalid json")]
    InvalidJson,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[async_trait]
pub trait AtcoderProblemsPort: Send + Sync {
    async fn fetch_problems(&self) -> Result<Vec<ApiProblem>, AtcoderProblemsError>;
}
