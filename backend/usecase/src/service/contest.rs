use async_trait::async_trait;
use domain::error::repository::RepositoryError;
use domain::model::problem::ContestSeries;

use crate::model::contests::ContestListItem;

#[async_trait]
pub trait ContestService: Send + Sync {
    async fn get_contents_by_series(
        &self,
        series: ContestSeries,
    ) -> Result<Vec<ContestListItem>, RepositoryError>;
}
