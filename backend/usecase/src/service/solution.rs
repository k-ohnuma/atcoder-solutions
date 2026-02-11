use async_trait::async_trait;
use domain::error::repository::RepositoryError;
use uuid::Uuid;

use crate::model::solution::{SolutionDetails, SolutionListItem};

#[async_trait]
pub trait SolutionService: Send + Sync {
    async fn get_solutions_by_problem_id(
        &self,
        problem_id: String,
    ) -> Result<Vec<SolutionListItem>, RepositoryError>;
    async fn get_solution_by_solution_id(
        &self,
        solution_id: Uuid,
    ) -> Result<SolutionDetails, RepositoryError>;
}
