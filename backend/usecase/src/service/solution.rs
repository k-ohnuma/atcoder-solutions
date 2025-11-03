use async_trait::async_trait;
use shared::error::repository::RepositoryError;

use crate::dto::solution::SolutionListItemView;


#[async_trait]
pub trait SolutionService: Send + Sync {
    async fn get_solutions_by_problem_id(
        &self,
        problem_id: String,
    ) -> Result<Vec<SolutionListItemView>, RepositoryError>;
}
