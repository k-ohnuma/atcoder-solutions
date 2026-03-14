use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use usecase::dto::solution::SolutionListItemView;
use uuid::Uuid;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLatestSolutionsRequest {
    pub size: Option<i32>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLatestSolutionsResponse {
    pub id: Uuid,
    pub title: String,
    pub problem_id: String,
    pub user_id: String,
    pub user_name: String,
    pub votes_count: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<SolutionListItemView> for GetLatestSolutionsResponse {
    fn from(value: SolutionListItemView) -> Self {
        let SolutionListItemView {
            id,
            title,
            problem_id,
            user_id,
            user_name,
            votes_count,
            created_at,
            updated_at,
        } = value;

        Self {
            id,
            title,
            problem_id,
            user_id,
            user_name,
            votes_count,
            created_at,
            updated_at,
        }
    }
}
