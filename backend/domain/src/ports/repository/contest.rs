use async_trait::async_trait;

use crate::{
    error::repository::RepositoryError, model::problem::ContestSeries,
    read_model::contests::ContestListItem,
};

#[async_trait]
pub trait ContestRepository: Send + Sync {
    async fn get_contests_by_series(
        &self,
        series: ContestSeries,
    ) -> Result<Vec<ContestListItem>, RepositoryError>;
}
