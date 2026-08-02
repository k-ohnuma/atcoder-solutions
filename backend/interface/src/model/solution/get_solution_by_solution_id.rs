use chrono::{DateTime, Utc};
use serde::Serialize;
use usecase::model::solution::output::SolutionOutput;
use uuid::Uuid;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSolutionBySolutionIdResponse {
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

impl From<SolutionOutput> for GetSolutionBySolutionIdResponse {
    fn from(value: SolutionOutput) -> Self {
        let SolutionOutput {
            id,
            title,
            problem_id,
            contest_code,
            problem_title,
            user_id,
            user_name,
            tags,
            body_md,
            submit_url,
            created_at,
            updated_at,
        } = value;

        Self {
            id: id.into_uuid(),
            title,
            problem_id: problem_id.into_string(),
            contest_code: contest_code.into_string(),
            problem_title: problem_title.into_string(),
            user_id,
            user_name,
            tags,
            body_md,
            submit_url,
            created_at,
            updated_at,
        }
    }
}
