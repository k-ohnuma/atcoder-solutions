use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnvoteSolutionRequest {
    pub solution_id: Uuid,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UnvoteSolutionResponse {
    pub solution_id: Uuid,
    pub liked: bool,
}

impl UnvoteSolutionResponse {
    pub fn unliked(solution_id: Uuid) -> Self {
        Self {
            solution_id,
            liked: false,
        }
    }
}
