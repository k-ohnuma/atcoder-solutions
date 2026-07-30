use std::sync::Arc;

use domain::ports::repository::contest::ContestRepository;

use crate::{
    error::ContestError,
    model::contests::{GetContestsBySeriesInput, output::ContestListItemOutput},
};
use derive_new::new;

#[derive(new)]
pub struct GetContestsBySeriesUsecase {
    repository: Arc<dyn ContestRepository>,
}

impl GetContestsBySeriesUsecase {
    pub async fn run(
        &self,
        input: GetContestsBySeriesInput,
    ) -> Result<Vec<ContestListItemOutput>, ContestError> {
        let items = self
            .repository
            .get_contests_by_series(input.series)
            .await
            .map_err(ContestError::from)?;
        Ok(items.into_iter().map(ContestListItemOutput::from).collect())
    }
}
