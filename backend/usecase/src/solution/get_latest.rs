use std::sync::Arc;

use derive_new::new;

use crate::{
    dto::solution::SolutionListItemView, model::solution::SolutionError,
    service::solution::SolutionService,
};

#[derive(new)]
pub struct GetLatestSolutionsUsecase {
    service: Arc<dyn SolutionService>,
}

impl GetLatestSolutionsUsecase {
    pub async fn run(&self, size: Option<i32>) -> Result<Vec<SolutionListItemView>, SolutionError> {
        let items = self.service.get_latest_solutions(size).await?;
        Ok(items.into_iter().map(SolutionListItemView::from).collect())
    }
}
