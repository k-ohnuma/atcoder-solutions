use std::sync::Arc;

use derive_new::new;
use domain::{
    model::{solution::id::SolutionId, user::id::UserId},
    ports::repository::solution::SolutionRepository,
};

use crate::{error::SolutionError, solution::authorization::ensure_solution_owner};

#[derive(new)]
pub struct DeleteSolutionUsecase {
    repository: Arc<dyn SolutionRepository>,
}

impl DeleteSolutionUsecase {
    pub async fn run(
        &self,
        user_id: UserId,
        solution_id: SolutionId,
    ) -> Result<SolutionId, SolutionError> {
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
