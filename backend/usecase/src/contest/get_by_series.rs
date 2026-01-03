use std::sync::Arc;

use derive_new::new;
use domain::model::problem::ContestSeries;

use crate::{
    dto::contests::ContestListItemView, model::contests::ContestError,
    service::contest::ContestService,
};

#[derive(new)]
pub struct GetContestsBySeriesUseCase {
    service: Arc<dyn ContestService>,
}

impl GetContestsBySeriesUseCase {
    pub async fn run(
        &self,
        series: ContestSeries,
    ) -> Result<Vec<ContestListItemView>, ContestError> {
        self.service
            .get_contents_by_series(series)
            .await
            .map_err(ContestError::from)
    }
}
