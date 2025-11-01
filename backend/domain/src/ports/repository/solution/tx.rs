use async_trait::async_trait;
use shared::error::repository::RepositoryError;
use uuid::Uuid;

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
pub trait UnitOfWork: Send + Sync {
    fn solutions(&mut self) -> &mut dyn SolutionRespositoryTx;
    fn tags(&mut self) -> &mut dyn TagRepositoryTx;

    async fn commit(self: Box<Self>) -> Result<(), RepositoryError>;
    async fn rollback(self: Box<Self>) -> Result<(), RepositoryError>;
}

#[async_trait]
pub trait SolutionTxManager: Send + Sync {
    async fn begin(&self) -> Result<Box<dyn UnitOfWork>, RepositoryError>;
}
