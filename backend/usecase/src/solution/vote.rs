use std::sync::Arc;

use derive_new::new;
use domain::{
    model::{solution::id::SolutionId, user::id::UserId},
    ports::repository::solution::SolutionRepository,
};

use crate::error::SolutionError;

#[derive(new)]
pub struct VoteSolutionUsecase {
    repository: Arc<dyn SolutionRepository>,
}

impl VoteSolutionUsecase {
    pub async fn run(&self, user_id: UserId, solution_id: SolutionId) -> Result<(), SolutionError> {
        let exists = self.repository.solution_exists(solution_id).await?;
        if !exists {
            return Err(SolutionError::NotFound("solution not found".to_string()));
        }

        self.repository.vote_solution(&user_id, solution_id).await?;
        Ok(())
    }
}
