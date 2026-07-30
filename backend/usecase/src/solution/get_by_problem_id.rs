use std::sync::Arc;

use derive_new::new;
use domain::ports::repository::solution::SolutionRepository;

use crate::{
    error::SolutionError,
    model::solution::{list::GetSolutionsByProblemIdInput, output::SolutionListItemOutput},
};

#[derive(new)]
pub struct GetSolutionsByProblemIdUsecase {
    repository: Arc<dyn SolutionRepository>,
}

impl GetSolutionsByProblemIdUsecase {
    pub async fn run(
        &self,
        input: GetSolutionsByProblemIdInput,
    ) -> Result<Vec<SolutionListItemOutput>, SolutionError> {
        let exists = self.repository.problem_exists(&input.problem_id).await?;
        if !exists {
            return Err(SolutionError::NotFound("problem not found".to_string()));
        }

        let items = self
            .repository
            .get_solutions_by_problem_id(input.problem_id, input.sort, input.limit)
            .await?;
        Ok(items
            .into_iter()
            .map(SolutionListItemOutput::from)
            .collect())
    }
}
