use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use usecase::dto::solution::SolutionView;
use uuid::Uuid;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSolutionBySolutionIdRequest {
    pub solution_id: Uuid,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSolutionBySolutionIdResponse {
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

impl From<SolutionView> for GetSolutionBySolutionIdResponse {
    fn from(value: SolutionView) -> Self {
        let SolutionView {
            id,
            title,
            problem_id,
            problem_title,
            user_id,
            user_name,
            body_md,
            submit_url,
            created_at,
            updated_at,
        } = value;

        Self {
            id,
            title,
            problem_id,
            problem_title,
            user_id,
            user_name,
            body_md,
            submit_url,
            created_at,
            updated_at,
        }
    }
}
