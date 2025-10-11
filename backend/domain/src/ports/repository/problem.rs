use async_trait::async_trait;
use shared::error::repository::RepositoryError;

use crate::model::problem::Problem;


#[async_trait]
pub trait ProblemRepository: Send + Sync {
    async fn create_records(&self, problems: Vec<Problem>) -> Result<(), RepositoryError>;
}

