use std::sync::Arc;

use derive_new::new;
use domain::ports::repository::solution::tx::SolutionTxManager;
use uuid::Uuid;

use crate::model::solution::SolutionError;

#[derive(new)]
pub struct UnvoteSolutionUsecase {
    txm: Arc<dyn SolutionTxManager>,
}

impl UnvoteSolutionUsecase {
    pub async fn run(&self, user_id: String, solution_id: Uuid) -> Result<(), SolutionError> {
        let mut uow = self.txm.begin().await?;
        uow.votes().unlike(&user_id, solution_id).await?;
        uow.commit().await?;
        Ok(())
    }
}
