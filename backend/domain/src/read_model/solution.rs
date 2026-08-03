use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::model::{
    problem::{contest_code::ContestCode, id::ProblemId, title::ProblemTitle},
    solution::id::SolutionId,
    user::id::UserId,
};

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
    pub user_name: String,
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
    pub user_name: String,
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
    pub user_name: String,
    pub tags: Vec<String>,
    pub body_md: String,
    pub submit_url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct SolutionComment {
    pub id: Uuid,
    pub user_id: UserId,
    pub user_name: String,
    pub solution_id: SolutionId,
    pub body_md: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
