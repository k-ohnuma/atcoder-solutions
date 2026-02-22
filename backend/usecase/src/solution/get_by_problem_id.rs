use std::sync::Arc;

use derive_new::new;

use crate::{
    dto::solution::SolutionListItemView,
    model::solution::{SolutionError, SolutionListSort},
    service::solution::SolutionService,
};

#[derive(new)]
pub struct GetSolutionsByProblemIdUsecase {
    service: Arc<dyn SolutionService>,
}

impl GetSolutionsByProblemIdUsecase {
    pub async fn run(
        &self,
        problem_id: String,
        sort: SolutionListSort,
    ) -> Result<Vec<SolutionListItemView>, SolutionError> {
        if problem_id.trim().is_empty() {
            return Err(SolutionError::BadRequest(
                "problem_id cannot be empty".to_string(),
            ));
        }

        let exists = self.service.problem_exists(&problem_id).await?;
        if !exists {
            return Err(SolutionError::NotFound("problem not found".to_string()));
        }

        let items = self
            .service
            .get_solutions_by_problem_id(problem_id, sort)
            .await?;
        Ok(items.into_iter().map(SolutionListItemView::from).collect())
    }
}
