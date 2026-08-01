use async_trait::async_trait;

use crate::error::repository::RepositoryError;
use crate::model::problem::Problem;

#[async_trait]
pub trait ProblemRepositoryTx: Send + Sync {
    async fn upsert_contests(
        &mut self,
        contests: &[(String, String)],
    ) -> Result<(), RepositoryError>;
    async fn upsert_problems(&mut self, problems: &[Problem]) -> Result<(), RepositoryError>;
}

#[async_trait]
pub trait ProblemUnitOfWork: Send + Sync {
    fn problems(&mut self) -> &mut dyn ProblemRepositoryTx;
    async fn commit(self: Box<Self>) -> Result<(), RepositoryError>;
}

#[async_trait]
pub trait ProblemTxManager: Send + Sync {
    async fn begin(&self) -> Result<Box<dyn ProblemUnitOfWork>, RepositoryError>;
}
