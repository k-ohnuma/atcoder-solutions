use chrono::{DateTime, Utc};
use uuid::Uuid;

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
