use std::sync::Arc;

use derive_new::new;

use crate::{
    dto::solution::SolutionListItemView, model::solution::SolutionError,
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
    ) -> Result<Vec<SolutionListItemView>, SolutionError> {
        let items = self.service.get_solutions_by_problem_id(problem_id).await?;
        Ok(items.into_iter().map(SolutionListItemView::from).collect())
    }
}
