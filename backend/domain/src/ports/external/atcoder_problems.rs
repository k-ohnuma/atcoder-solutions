use async_trait::async_trait;

use crate::error::atcoder_problems::AtcoderProblemsError;
use crate::model::problem::{Problem, difficulty::Difficulty, id::ProblemId};

#[async_trait]
pub trait AtcoderProblemsPort: Send + Sync {
    async fn fetch_problems(&self) -> Result<Vec<Problem>, AtcoderProblemsError>;
    async fn fetch_difficulty(
        &self,
        problem_id: &ProblemId,
    ) -> Result<Option<Difficulty>, AtcoderProblemsError>;
}
