use serde::{Deserialize, Serialize};
use usecase::model::solution::create::CreateSolutionInput;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateSolutionRequest {
    problem_id: String,
    body_md: String,
    submit_url: String,
    tags: Vec<String>,
}

pub fn from_req_for_input(user_id: String, req: CreateSolutionRequest) -> CreateSolutionInput {
    CreateSolutionInput {
        user_id,
        problem_id: req.problem_id,
        body_md: req.body_md,
        submit_url: req.submit_url,
        tags: req.tags,
    }
}

#[derive(Serialize)]
pub struct CreateSolutionResponse {
    solution_id: Uuid,
}

impl From<Uuid> for CreateSolutionResponse {
    fn from(value: Uuid) -> Self {
        Self { solution_id: value }
    }
}
