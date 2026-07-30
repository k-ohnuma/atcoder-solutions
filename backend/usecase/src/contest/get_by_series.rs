use std::sync::Arc;

use crate::{
    dto::contests::ContestListItemView,
    model::contests::{ContestError, GetContestsBySeriesInput},
    service::contest::ContestService,
};
use derive_new::new;

#[derive(new)]
pub struct GetContestsBySeriesUseCase {
    service: Arc<dyn ContestService>,
}

impl GetContestsBySeriesUseCase {
    pub async fn run(
        &self,
        input: GetContestsBySeriesInput,
    ) -> Result<Vec<ContestListItemView>, ContestError> {
        let items = self
            .service
            .get_contents_by_series(input.series)
            .await
            .map_err(ContestError::from)?;
        Ok(items.into_iter().map(ContestListItemView::from).collect())
    }
}
