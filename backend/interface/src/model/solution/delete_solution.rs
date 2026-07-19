use serde::Serialize;
use uuid::Uuid;

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
