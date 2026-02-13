use async_trait::async_trait;
use domain::error::repository::RepositoryError;
use uuid::Uuid;

use crate::model::solution::{
    SolutionComment, SolutionDetails, SolutionListItem, SolutionListSort,
};

#[async_trait]
pub trait SolutionService: Send + Sync {
    async fn get_solutions_by_problem_id(
        &self,
        problem_id: String,
        sort: SolutionListSort,
    ) -> Result<Vec<SolutionListItem>, RepositoryError>;
    async fn get_solution_by_solution_id(
        &self,
        solution_id: Uuid,
    ) -> Result<SolutionDetails, RepositoryError>;
    async fn get_solution_votes_count(&self, solution_id: Uuid) -> Result<i64, RepositoryError>;
    async fn has_user_voted_solution(
        &self,
        user_id: String,
        solution_id: Uuid,
    ) -> Result<bool, RepositoryError>;
    async fn solution_exists(&self, solution_id: Uuid) -> Result<bool, RepositoryError>;
    async fn get_comments_by_solution_id(
        &self,
        solution_id: Uuid,
    ) -> Result<Vec<SolutionComment>, RepositoryError>;
    async fn get_user_name_by_id(&self, user_id: &str) -> Result<String, RepositoryError>;
}
