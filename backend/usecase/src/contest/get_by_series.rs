use std::sync::Arc;

use domain::ports::repository::contest::ContestRepository;

use crate::{
    dto::contests::ContestListItemView, error::ContestError,
    model::contests::GetContestsBySeriesInput,
};
use derive_new::new;

#[derive(new)]
pub struct GetContestsBySeriesUseCase {
    repository: Arc<dyn ContestRepository>,
}

impl GetContestsBySeriesUseCase {
    pub async fn run(
        &self,
        input: GetContestsBySeriesInput,
    ) -> Result<Vec<ContestListItemView>, ContestError> {
        let items = self
            .repository
            .get_contests_by_series(input.series)
            .await
            .map_err(ContestError::from)?;
        Ok(items.into_iter().map(ContestListItemView::from).collect())
    }
}
