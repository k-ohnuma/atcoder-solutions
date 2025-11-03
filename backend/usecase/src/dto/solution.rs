use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct SolutionListItemView {
    id: Uuid,
    problem_id: String,
    user_id: String,
    user_name: String,
    submit_url: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>
}

