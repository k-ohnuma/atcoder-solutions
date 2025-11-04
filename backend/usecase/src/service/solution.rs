use async_trait::async_trait;
use shared::error::repository::RepositoryError;
use uuid::Uuid;

use crate::dto::solution::{SolutionListItemView, SolutionView};

#[async_trait]
pub trait SolutionService: Send + Sync {
    async fn get_solutions_by_problem_id(
        &self,
        problem_id: String,
    ) -> Result<Vec<SolutionListItemView>, RepositoryError>;
    async fn get_solution_by_solution_id(
        &self,
        solution_id: Uuid,
    ) -> Result<SolutionView, RepositoryError>;
}
