use std::sync::Arc;

use derive_new::new;
use domain::ports::repository::solution::{SolutionRepository, tx::SolutionTxManager};
use uuid::Uuid;

use crate::error::SolutionError;

#[derive(new)]
pub struct UnvoteSolutionUsecase {
    txm: Arc<dyn SolutionTxManager>,
    repository: Arc<dyn SolutionRepository>,
}

impl UnvoteSolutionUsecase {
    pub async fn run(&self, user_id: String, solution_id: Uuid) -> Result<(), SolutionError> {
        let exists = self.repository.solution_exists(solution_id).await?;
        if !exists {
            return Err(SolutionError::BadRequest("solution not found".to_string()));
        }

        let mut uow = self.txm.begin().await?;
        uow.votes().unlike(&user_id, solution_id).await?;
        uow.commit().await?;
        Ok(())
    }
}
