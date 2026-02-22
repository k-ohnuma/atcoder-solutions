use std::sync::Arc;

use derive_new::new;
use domain::ports::repository::solution::tx::SolutionTxManager;
use uuid::Uuid;

use crate::{model::solution::SolutionError, service::solution::SolutionService};

#[derive(new)]
pub struct DeleteSolutionUsecase {
    txm: Arc<dyn SolutionTxManager>,
    service: Arc<dyn SolutionService>,
}

impl DeleteSolutionUsecase {
    pub async fn run(&self, user_id: String, solution_id: Uuid) -> Result<Uuid, SolutionError> {
        let exists = self.service.solution_exists(solution_id).await?;
        if !exists {
            return Err(SolutionError::NotFound("solution not found".to_string()));
        }

        let owner_user_id = self.service.get_solution_user_id(solution_id).await?;
        if owner_user_id != user_id {
            return Err(SolutionError::Forbidden(
                "you cannot delete this solution".to_string(),
            ));
        }

        let mut uow = self.txm.begin().await?;
        uow.solutions().delete(solution_id).await?;
        uow.commit().await?;

        Ok(solution_id)
    }
}
