use std::sync::Arc;

use derive_new::new;
use domain::ports::repository::solution::SolutionRepository;
use uuid::Uuid;

use crate::error::SolutionError;

#[derive(new)]
pub struct GetMyVoteStatusUsecase {
    repository: Arc<dyn SolutionRepository>,
}

impl GetMyVoteStatusUsecase {
    pub async fn run(&self, user_id: String, solution_id: Uuid) -> Result<bool, SolutionError> {
        let exists = self.repository.solution_exists(solution_id).await?;
        if !exists {
            return Err(SolutionError::NotFound("solution not found".to_string()));
        }

        let voted = self
            .repository
            .has_user_voted_solution(user_id, solution_id)
            .await?;
        Ok(voted)
    }
}
