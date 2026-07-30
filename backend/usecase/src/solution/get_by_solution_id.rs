use std::sync::Arc;

use derive_new::new;
use domain::ports::repository::solution::SolutionRepository;
use uuid::Uuid;

use crate::{error::SolutionError, model::solution::output::SolutionOutput};

#[derive(new)]
pub struct GetSolutionBySolutionIdUsecase {
    repository: Arc<dyn SolutionRepository>,
}

impl GetSolutionBySolutionIdUsecase {
    pub async fn run(&self, solution_id: Uuid) -> Result<SolutionOutput, SolutionError> {
        let item = self
            .repository
            .get_solution_by_solution_id(solution_id)
            .await?;
        Ok(item.into())
    }
}
