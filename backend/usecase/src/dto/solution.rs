use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::model::solution::{SolutionDetails, SolutionListItem};

pub struct SolutionListItemView {
    pub id: Uuid,
    pub title: String,
    pub problem_id: String,
    pub user_id: String,
    pub user_name: String,
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
    pub body_md: String,
    pub submit_url: String,
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
            body_md: value.body_md,
            submit_url: value.submit_url,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
