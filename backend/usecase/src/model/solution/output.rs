use chrono::{DateTime, Utc};
use domain::read_model::solution::{
    SolutionComment, SolutionDetails, SolutionListItem, UserSolutionListItem,
};
use uuid::Uuid;

pub struct SolutionListItemOutput {
    pub id: Uuid,
    pub title: String,
    pub problem_id: String,
    pub user_id: String,
    pub user_name: String,
    pub votes_count: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct SolutionOutput {
    pub id: Uuid,
    pub title: String,
    pub problem_id: String,
    pub contest_code: String,
    pub problem_title: String,
    pub user_id: String,
    pub user_name: String,
    pub tags: Vec<String>,
    pub body_md: String,
    pub submit_url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct UserSolutionListItemOutput {
    pub id: Uuid,
    pub title: String,
    pub problem_id: String,
    pub problem_title: String,
    pub user_id: String,
    pub user_name: String,
    pub votes_count: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct CommentOutput {
    pub id: Uuid,
    pub user_id: String,
    pub user_name: String,
    pub solution_id: Uuid,
    pub body_md: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<SolutionListItem> for SolutionListItemOutput {
    fn from(value: SolutionListItem) -> Self {
        Self {
            id: value.id,
            title: value.title,
            problem_id: value.problem_id,
            user_id: value.user_id,
            user_name: value.user_name,
            votes_count: value.votes_count,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl From<SolutionDetails> for SolutionOutput {
    fn from(value: SolutionDetails) -> Self {
        Self {
            id: value.id,
            title: value.title,
            problem_id: value.problem_id,
            contest_code: value.contest_code,
            problem_title: value.problem_title,
            user_id: value.user_id,
            user_name: value.user_name,
            tags: value.tags,
            body_md: value.body_md,
            submit_url: value.submit_url,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl From<UserSolutionListItem> for UserSolutionListItemOutput {
    fn from(value: UserSolutionListItem) -> Self {
        Self {
            id: value.id,
            title: value.title,
            problem_id: value.problem_id,
            problem_title: value.problem_title,
            user_id: value.user_id,
            user_name: value.user_name,
            votes_count: value.votes_count,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl From<SolutionComment> for CommentOutput {
    fn from(value: SolutionComment) -> Self {
        Self {
            id: value.id,
            user_id: value.user_id,
            user_name: value.user_name,
            solution_id: value.solution_id,
            body_md: value.body_md,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
