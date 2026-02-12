use std::sync::Arc;

use derive_new::new;
use domain::ports::repository::solution::tx::SolutionTxManager;
use uuid::Uuid;

use crate::model::solution::SolutionError;

#[derive(new)]
pub struct VoteSolutionUsecase {
    txm: Arc<dyn SolutionTxManager>,
}

impl VoteSolutionUsecase {
    pub async fn run(&self, user_id: String, solution_id: Uuid) -> Result<(), SolutionError> {
        let mut uow = self.txm.begin().await?;
        uow.votes().like(&user_id, solution_id).await?;
        uow.commit().await?;
        Ok(())
    }
}
