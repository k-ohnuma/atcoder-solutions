use chrono::{DateTime, Utc};
use usecase::dto::solution::SolutionListItemView;
use uuid::Uuid;

pub struct SolutionListItemViewRaw {
    pub id: Uuid,
    pub title: String,
    pub problem_id: String,
    pub user_id: String,
    pub user_name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<SolutionListItemViewRaw> for SolutionListItemView {
    fn from(value: SolutionListItemViewRaw) -> Self {
        let SolutionListItemViewRaw {
            id,
            title,
            problem_id,
            user_id,
            user_name,
            created_at,
            updated_at,
        } = value;

        Self {
            id,
            title,
            problem_id,
            user_id,
            user_name,
            created_at,
            updated_at,
        }
    }
}
