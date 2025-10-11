use async_trait::async_trait;
use shared::error::external::ExternalError;

use crate::model::atcoder_problems::ApiProblem;

#[async_trait]
pub trait AtcoderProblemsPort: Send + Sync {
    async fn fetch_problems(&self) -> Result<Vec<ApiProblem>, ExternalError>;
}
