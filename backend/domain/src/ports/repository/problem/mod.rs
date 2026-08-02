pub mod tx;

use async_trait::async_trait;

use crate::error::repository::RepositoryError;
use crate::model::problem::{ContestSeries, Problem, id::ProblemId};

#[async_trait]
pub trait ProblemRepository: Send + Sync {
    async fn get_problem_ids_with_difficulty(
        &self,
        problem_ids: &[ProblemId],
    ) -> Result<Vec<ProblemId>, RepositoryError>;
    async fn get_contest_codes_by_series(
        &self,
        series: ContestSeries,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<String>, RepositoryError>;
    async fn get_problems_by_contest_codes(
        &self,
        contest_codes: &[String],
    ) -> Result<Vec<Problem>, RepositoryError>;
    async fn search_problems_by_contest_series(
        &self,
        series: ContestSeries,
        query: &str,
    ) -> Result<Vec<Problem>, RepositoryError>;
    async fn get_problem_by_id(&self, problem_id: &ProblemId) -> Result<Problem, RepositoryError>;
    async fn problem_exists(&self, problem_id: &ProblemId) -> Result<bool, RepositoryError>;
    async fn get_problems_by_contest(&self, contest: &str)
    -> Result<Vec<Problem>, RepositoryError>;
}
