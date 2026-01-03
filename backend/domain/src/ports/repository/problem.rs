use async_trait::async_trait;
use shared::error::repository::RepositoryError;

use crate::model::problem::{ContestSeries, Problem};

#[async_trait]
pub trait ProblemRepository: Send + Sync {
    async fn create_records(&self, problems: Vec<Problem>) -> Result<(), RepositoryError>;
    async fn get_problems_by_contest_series(
        &self,
        series: ContestSeries,
    ) -> Result<Vec<Problem>, RepositoryError>;
    async fn get_problems_by_contest(&self, contest: &str) -> Result<Vec<Problem>, RepositoryError>;
}
