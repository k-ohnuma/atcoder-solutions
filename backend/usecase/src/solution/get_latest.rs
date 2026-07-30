use std::sync::Arc;

use derive_new::new;

use domain::ports::repository::solution::SolutionRepository;

use crate::{
    dto::solution::SolutionListItemView, error::SolutionError,
    model::solution::list::GetLatestSolutionsInput,
};

#[derive(new)]
pub struct GetLatestSolutionsUsecase {
    repository: Arc<dyn SolutionRepository>,
}

impl GetLatestSolutionsUsecase {
    pub async fn run(
        &self,
        input: GetLatestSolutionsInput,
    ) -> Result<Vec<SolutionListItemView>, SolutionError> {
        let items = self.repository.get_latest_solutions(input.limit).await?;
        Ok(items.into_iter().map(SolutionListItemView::from).collect())
    }
}
