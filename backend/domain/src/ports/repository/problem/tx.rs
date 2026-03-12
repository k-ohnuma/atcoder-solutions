use async_trait::async_trait;

use crate::error::repository::RepositoryError;
use crate::model::problem::Problem;

#[async_trait]
pub trait ProblemRepositoryTx: Send + Sync {
    async fn upsert_contest(
        &mut self,
        contest_code: &str,
        series_code: &str,
    ) -> Result<(), RepositoryError>;
    async fn upsert_problem(
        &mut self,
        problem_id: &str,
        contest_code: &str,
        problem_index: &str,
        title: &str,
        difficulty: Option<i32>,
    ) -> Result<(), RepositoryError>;
    async fn upsert_contests_bulk(
        &mut self,
        contests: &[(String, String)],
    ) -> Result<(), RepositoryError>;
    async fn upsert_problems_bulk(&mut self, problems: &[Problem]) -> Result<(), RepositoryError>;
}

#[async_trait]
pub trait ProblemUnitOfWork: Send + Sync {
    fn problems(&mut self) -> &mut dyn ProblemRepositoryTx;
    async fn commit(self: Box<Self>) -> Result<(), RepositoryError>;
    async fn rollback(self: Box<Self>) -> Result<(), RepositoryError>;
}

#[async_trait]
pub trait ProblemTxManager: Send + Sync {
    async fn begin(&self) -> Result<Box<dyn ProblemUnitOfWork>, RepositoryError>;
}
