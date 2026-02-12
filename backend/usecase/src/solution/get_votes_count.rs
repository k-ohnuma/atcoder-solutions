use std::sync::Arc;

use derive_new::new;
use uuid::Uuid;

use crate::{model::solution::SolutionError, service::solution::SolutionService};

#[derive(new)]
pub struct GetSolutionVotesCountUsecase {
    service: Arc<dyn SolutionService>,
}

impl GetSolutionVotesCountUsecase {
    pub async fn run(&self, solution_id: Uuid) -> Result<i64, SolutionError> {
        let count = self.service.get_solution_votes_count(solution_id).await?;
        Ok(count)
    }
}
