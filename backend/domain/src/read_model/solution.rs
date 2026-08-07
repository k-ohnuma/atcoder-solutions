use crate::model::{
    problem::{contest_code::ContestCode, id::ProblemId, title::ProblemTitle},
    solution::{comment_id::CommentId, id::SolutionId},
    user::{id::UserId, name::UserName},
};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SolutionListSort {
    Latest,
    Votes,
}

#[derive(Debug, Clone)]
pub struct SolutionListItem {
    pub id: SolutionId,
    pub title: String,
    pub problem_id: ProblemId,
    pub user_id: UserId,
    pub user_name: UserName,
    pub votes_count: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct UserSolutionListItem {
    pub id: SolutionId,
    pub title: String,
    pub problem_id: ProblemId,
    pub problem_title: ProblemTitle,
    pub user_id: UserId,
    pub user_name: UserName,
    pub votes_count: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct SolutionDetails {
    pub id: SolutionId,
    pub title: String,
    pub problem_id: ProblemId,
    pub contest_code: ContestCode,
    pub problem_title: ProblemTitle,
    pub user_id: UserId,
    pub user_name: UserName,
    pub tags: Vec<String>,
    pub body_md: String,
    pub submit_url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct SolutionComment {
    pub id: CommentId,
    pub user_id: UserId,
    pub user_name: UserName,
    pub solution_id: SolutionId,
    pub body_md: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
