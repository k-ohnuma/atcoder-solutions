use async_trait::async_trait;
use uuid::Uuid;

use crate::error::repository::RepositoryError;
use crate::model::solution::Solution;

#[async_trait]
pub trait SolutionRespositoryTx: Send + Sync {
    async fn create(&mut self, s: &Solution) -> Result<Uuid, RepositoryError>;
    async fn replace_tags(
        &mut self,
        solution_id: Uuid,
        tag_id: &[Uuid],
    ) -> Result<(), RepositoryError>;
}

#[async_trait]
pub trait TagRepositoryTx: Send + Sync {
    async fn upsert(&mut self, names: &[String]) -> Result<Vec<Uuid>, RepositoryError>;
}

#[async_trait]
pub trait VoteRepositoryTx: Send + Sync {
    async fn like(&mut self, user_id: &str, solution_id: Uuid) -> Result<(), RepositoryError>;
    async fn unlike(&mut self, user_id: &str, solution_id: Uuid) -> Result<(), RepositoryError>;
}

#[async_trait]
pub trait UnitOfWork: Send + Sync {
    fn solutions(&mut self) -> &mut dyn SolutionRespositoryTx;
    fn tags(&mut self) -> &mut dyn TagRepositoryTx;
    fn votes(&mut self) -> &mut dyn VoteRepositoryTx;

    async fn commit(self: Box<Self>) -> Result<(), RepositoryError>;
    async fn rollback(self: Box<Self>) -> Result<(), RepositoryError>;
}

#[async_trait]
pub trait SolutionTxManager: Send + Sync {
    async fn begin(&self) -> Result<Box<dyn UnitOfWork>, RepositoryError>;
}
