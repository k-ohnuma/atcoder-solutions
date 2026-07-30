use std::sync::Arc;

use derive_new::new;
use domain::ports::repository::solution::SolutionRepository;
use uuid::Uuid;

use crate::{error::SolutionError, solution::authorization::ensure_solution_owner};

#[derive(new)]
pub struct DeleteSolutionUsecase {
    repository: Arc<dyn SolutionRepository>,
}

impl DeleteSolutionUsecase {
    pub async fn run(&self, user_id: String, solution_id: Uuid) -> Result<Uuid, SolutionError> {
        ensure_solution_owner(
            self.repository.as_ref(),
            solution_id,
            &user_id,
            "you cannot delete this solution",
        )
        .await?;

        self.repository.delete_solution(solution_id).await?;

        Ok(solution_id)
    }
}
