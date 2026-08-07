use crate::{
    error::repository::RepositoryError,
    model::{
        problem::id::ProblemId,
        solution::{comment_id::CommentId, id::SolutionId},
        user::{id::UserId, name::UserName},
    },
    read_model::solution::{
        SolutionComment, SolutionDetails, SolutionListItem, SolutionListSort, UserSolutionListItem,
    },
};
use async_trait::async_trait;

pub mod tx;

#[async_trait]
pub trait SolutionRepository: Send + Sync {
    async fn delete_solution(&self, solution_id: SolutionId) -> Result<(), RepositoryError>;
    async fn vote_solution(
        &self,
        user_id: &UserId,
        solution_id: SolutionId,
    ) -> Result<(), RepositoryError>;
    async fn unvote_solution(
        &self,
        user_id: &UserId,
        solution_id: SolutionId,
    ) -> Result<(), RepositoryError>;
    async fn create_comment(
        &self,
        user_id: &UserId,
        solution_id: SolutionId,
        body_md: &str,
    ) -> Result<SolutionComment, RepositoryError>;
    async fn update_comment(
        &self,
        comment_id: CommentId,
        body_md: &str,
    ) -> Result<SolutionComment, RepositoryError>;
    async fn delete_comment(&self, comment_id: CommentId) -> Result<(), RepositoryError>;
    async fn get_latest_solutions(
        &self,
        limit: Option<i32>,
    ) -> Result<Vec<SolutionListItem>, RepositoryError>;
    async fn get_solutions_by_problem_id(
        &self,
        problem_id: ProblemId,
        sort: SolutionListSort,
        limit: Option<i32>,
    ) -> Result<Vec<SolutionListItem>, RepositoryError>;
    async fn get_solution_by_solution_id(
        &self,
        solution_id: SolutionId,
    ) -> Result<SolutionDetails, RepositoryError>;
    async fn get_solutions_by_user_name(
        &self,
        user_name: UserName,
        sort: SolutionListSort,
    ) -> Result<Vec<UserSolutionListItem>, RepositoryError>;
    async fn get_solution_votes_count(
        &self,
        solution_id: SolutionId,
    ) -> Result<i64, RepositoryError>;
    async fn has_user_voted_solution(
        &self,
        user_id: &UserId,
        solution_id: SolutionId,
    ) -> Result<bool, RepositoryError>;
    async fn solution_exists(&self, solution_id: SolutionId) -> Result<bool, RepositoryError>;
    async fn get_comments_by_solution_id(
        &self,
        solution_id: SolutionId,
    ) -> Result<Vec<SolutionComment>, RepositoryError>;
    async fn get_solution_user_id(
        &self,
        solution_id: SolutionId,
    ) -> Result<UserId, RepositoryError>;
    async fn comment_exists(&self, comment_id: CommentId) -> Result<bool, RepositoryError>;
    async fn get_comment_user_id(&self, comment_id: CommentId) -> Result<UserId, RepositoryError>;
}
