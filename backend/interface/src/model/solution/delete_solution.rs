use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteSolutionRequest {
    pub solution_id: Uuid,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteSolutionResponse {
    pub solution_id: Uuid,
}

impl From<Uuid> for DeleteSolutionResponse {
    fn from(value: Uuid) -> Self {
        Self { solution_id: value }
    }
}
