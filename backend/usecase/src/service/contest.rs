use async_trait::async_trait;
use domain::model::problem::ContestSeries;
use shared::error::repository::RepositoryError;

use crate::dto::contests::ContestListItemView;

#[async_trait]
pub trait ContestService: Send + Sync {
    async fn get_contents_by_series(
        &self,
        series: ContestSeries,
    ) -> Result<Vec<ContestListItemView>, RepositoryError>;
}
