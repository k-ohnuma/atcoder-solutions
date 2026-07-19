use serde::Serialize;
use uuid::Uuid;

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
