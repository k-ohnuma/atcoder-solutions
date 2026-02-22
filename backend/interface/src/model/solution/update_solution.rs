use serde::{Deserialize, Serialize};
use usecase::model::solution::update::UpdateSolutionInput;
use uuid::Uuid;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSolutionRequest {
    pub solution_id: Uuid,
    pub title: String,
    pub body_md: String,
    pub submit_url: String,
    pub tags: Vec<String>,
}

pub fn from_req_for_input(user_id: String, req: UpdateSolutionRequest) -> UpdateSolutionInput {
    UpdateSolutionInput {
        user_id,
        solution_id: req.solution_id,
        title: req.title,
        body_md: req.body_md,
        submit_url: req.submit_url,
        tags: req.tags,
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSolutionResponse {
    pub solution_id: Uuid,
}

impl From<Uuid> for UpdateSolutionResponse {
    fn from(value: Uuid) -> Self {
        Self { solution_id: value }
    }
}
