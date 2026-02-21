use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::model::solution::{CreatedComment, SolutionComment, SolutionDetails, SolutionListItem};

pub struct SolutionListItemView {
    pub id: Uuid,
    pub title: String,
    pub problem_id: String,
    pub user_id: String,
    pub user_name: String,
    pub votes_count: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct SolutionView {
    pub id: Uuid,
    pub title: String,
    pub problem_id: String,
    pub problem_title: String,
    pub user_id: String,
    pub user_name: String,
    pub tags: Vec<String>,
    pub body_md: String,
    pub submit_url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct CreatedCommentView {
    pub id: Uuid,
    pub user_id: String,
    pub user_name: String,
    pub solution_id: Uuid,
    pub body_md: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct SolutionCommentView {
    pub id: Uuid,
    pub user_id: String,
    pub user_name: String,
    pub solution_id: Uuid,
    pub body_md: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<SolutionListItem> for SolutionListItemView {
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

impl From<SolutionDetails> for SolutionView {
    fn from(value: SolutionDetails) -> Self {
        Self {
            id: value.id,
            title: value.title,
            problem_id: value.problem_id,
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

impl From<CreatedComment> for CreatedCommentView {
    fn from(value: CreatedComment) -> Self {
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

impl From<SolutionComment> for SolutionCommentView {
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
