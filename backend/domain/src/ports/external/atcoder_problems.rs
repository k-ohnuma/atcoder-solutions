use async_trait::async_trait;

use crate::error::external::ExternalError;
use crate::model::problem::Problem;

#[async_trait]
pub trait AtcoderProblemsPort: Send + Sync {
    async fn fetch_problems(&self) -> Result<Vec<Problem>, ExternalError>;
    async fn fetch_difficulty(&self, problem_id: &str) -> Result<Option<i32>, ExternalError>;
}
