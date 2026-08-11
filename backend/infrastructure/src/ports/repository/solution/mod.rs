mod comment;
mod delete;
mod read;
pub mod tx;
mod vote;

use async_trait::async_trait;
use derive_new::new;
use domain::{
    error::repository::RepositoryError,
    model::{
        problem::id::ProblemId,
        solution::{comment_body::CommentBody, comment_id::CommentId, id::SolutionId},
        user::{id::UserId, name::UserName},
    },
    ports::repository::solution::SolutionRepository,
    read_model::solution::{
        SolutionComment, SolutionDetails, SolutionListItem, SolutionListSort, UserSolutionListItem,
    },
};

use crate::database::ConnectionPool;

#[derive(new)]
pub struct SolutionRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl SolutionRepository for SolutionRepositoryImpl {
    async fn delete_solution(&self, solution_id: SolutionId) -> Result<(), RepositoryError> {
        delete::delete_solution(&self.db, solution_id).await
    }

    async fn vote_solution(
        &self,
        user_id: &UserId,
        solution_id: SolutionId,
    ) -> Result<(), RepositoryError> {
        vote::vote_solution(&self.db, user_id, solution_id).await
    }

    async fn unvote_solution(
        &self,
        user_id: &UserId,
        solution_id: SolutionId,
    ) -> Result<(), RepositoryError> {
        vote::unvote_solution(&self.db, user_id, solution_id).await
    }

    async fn create_comment(
        &self,
        user_id: &UserId,
        solution_id: SolutionId,
        body: &CommentBody,
    ) -> Result<SolutionComment, RepositoryError> {
        comment::create_comment(&self.db, user_id, solution_id, body).await
    }

    async fn update_comment(
        &self,
        comment_id: CommentId,
        body: &CommentBody,
    ) -> Result<SolutionComment, RepositoryError> {
        comment::update_comment(&self.db, comment_id, body).await
    }

    async fn delete_comment(&self, comment_id: CommentId) -> Result<(), RepositoryError> {
        comment::delete_comment(&self.db, comment_id).await
    }

    async fn get_latest_solutions(
        &self,
        limit: Option<i32>,
    ) -> Result<Vec<SolutionListItem>, RepositoryError> {
        read::get_latest_solutions(&self.db, limit).await
    }

    async fn get_solutions_by_problem_id(
        &self,
        problem_id: ProblemId,
        sort: SolutionListSort,
        limit: Option<i32>,
    ) -> Result<Vec<SolutionListItem>, RepositoryError> {
        read::get_solutions_by_problem_id(&self.db, problem_id, sort, limit).await
    }

    async fn get_solution_by_solution_id(
        &self,
        solution_id: SolutionId,
    ) -> Result<Option<SolutionDetails>, RepositoryError> {
        read::get_solution_by_solution_id(&self.db, solution_id).await
    }

    async fn get_solutions_by_user_name(
        &self,
        user_name: UserName,
        sort: SolutionListSort,
    ) -> Result<Vec<UserSolutionListItem>, RepositoryError> {
        read::get_solutions_by_user_name(&self.db, user_name, sort).await
    }

    async fn get_solution_votes_count(
        &self,
        solution_id: SolutionId,
    ) -> Result<i64, RepositoryError> {
        vote::get_solution_votes_count(&self.db, solution_id).await
    }

    async fn has_user_voted_solution(
        &self,
        user_id: &UserId,
        solution_id: SolutionId,
    ) -> Result<bool, RepositoryError> {
        vote::has_user_voted_solution(&self.db, user_id, solution_id).await
    }

    async fn solution_exists(&self, solution_id: SolutionId) -> Result<bool, RepositoryError> {
        read::solution_exists(&self.db, solution_id).await
    }

    async fn get_comments_by_solution_id(
        &self,
        solution_id: SolutionId,
    ) -> Result<Vec<SolutionComment>, RepositoryError> {
        comment::get_comments_by_solution_id(&self.db, solution_id).await
    }

    async fn get_solution_user_id(
        &self,
        solution_id: SolutionId,
    ) -> Result<Option<UserId>, RepositoryError> {
        read::get_solution_user_id(&self.db, solution_id).await
    }

    async fn comment_exists(&self, comment_id: CommentId) -> Result<bool, RepositoryError> {
        comment::comment_exists(&self.db, comment_id).await
    }

    async fn get_comment_user_id(
        &self,
        comment_id: CommentId,
    ) -> Result<Option<UserId>, RepositoryError> {
        comment::get_comment_user_id(&self.db, comment_id).await
    }
}
