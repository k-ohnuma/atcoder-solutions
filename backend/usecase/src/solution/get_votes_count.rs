use std::sync::Arc;

use derive_new::new;
use domain::{model::solution::id::SolutionId, ports::repository::solution::SolutionRepository};

use crate::error::SolutionError;

#[derive(new)]
pub struct GetSolutionVotesCountUsecase {
    repository: Arc<dyn SolutionRepository>,
}

impl GetSolutionVotesCountUsecase {
    pub async fn run(&self, solution_id: SolutionId) -> Result<i64, SolutionError> {
        let exists = self.repository.solution_exists(solution_id).await?;
        if !exists {
            return Err(SolutionError::NotFound("solution not found".to_string()));
        }

        let count = self
            .repository
            .get_solution_votes_count(solution_id)
            .await?;
        Ok(count)
    }
}
