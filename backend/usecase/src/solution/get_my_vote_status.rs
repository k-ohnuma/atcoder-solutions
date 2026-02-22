use std::sync::Arc;

use derive_new::new;
use uuid::Uuid;

use crate::{model::solution::SolutionError, service::solution::SolutionService};

#[derive(new)]
pub struct GetMyVoteStatusUsecase {
    service: Arc<dyn SolutionService>,
}

impl GetMyVoteStatusUsecase {
    pub async fn run(&self, user_id: String, solution_id: Uuid) -> Result<bool, SolutionError> {
        let exists = self.service.solution_exists(solution_id).await?;
        if !exists {
            return Err(SolutionError::NotFound("solution not found".to_string()));
        }

        let voted = self
            .service
            .has_user_voted_solution(user_id, solution_id)
            .await?;
        Ok(voted)
    }
}
