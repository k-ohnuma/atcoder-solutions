use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VoteSolutionRequest {
    pub solution_id: Uuid,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VoteSolutionResponse {
    pub solution_id: Uuid,
    pub liked: bool,
}

impl VoteSolutionResponse {
    pub fn liked(solution_id: Uuid) -> Self {
        Self {
            solution_id,
            liked: true,
        }
    }
}
