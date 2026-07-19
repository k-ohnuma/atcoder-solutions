use serde::Serialize;
use uuid::Uuid;

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
