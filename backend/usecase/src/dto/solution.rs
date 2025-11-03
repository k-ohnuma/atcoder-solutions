use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct SolutionListItemView {
    pub id: Uuid,
    pub problem_id: String,
    pub user_id: String,
    pub user_name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}

