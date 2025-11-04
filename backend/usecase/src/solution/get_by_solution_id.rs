use std::sync::Arc;

use derive_new::new;
use uuid::Uuid;

use crate::{dto::solution::SolutionView, model::solution::SolutionError, service::solution::SolutionService};


#[derive(new)]
pub struct GetSolutionBySolutionIdUsecase {
    service: Arc<dyn SolutionService>,
}

impl GetSolutionBySolutionIdUsecase {
    pub async fn run(
        &self,
        solution_id: Uuid,
    ) -> Result<SolutionView, SolutionError> {
        let item = self.service.get_solution_by_solution_id(solution_id).await?;
        Ok(item)
    }
}

