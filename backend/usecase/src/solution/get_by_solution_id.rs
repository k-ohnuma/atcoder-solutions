use std::sync::Arc;

use derive_new::new;
use domain::{model::solution::id::SolutionId, ports::repository::solution::SolutionRepository};

use crate::{error::SolutionError, model::solution::output::SolutionOutput};

#[derive(new)]
pub struct GetSolutionBySolutionIdUsecase {
    repository: Arc<dyn SolutionRepository>,
}

impl GetSolutionBySolutionIdUsecase {
    pub async fn run(&self, solution_id: SolutionId) -> Result<SolutionOutput, SolutionError> {
        let item = self
            .repository
            .get_solution_by_solution_id(solution_id)
            .await?
            .ok_or_else(|| SolutionError::NotFound("solution not found".to_string()))?;
        Ok(item.into())
    }
}
