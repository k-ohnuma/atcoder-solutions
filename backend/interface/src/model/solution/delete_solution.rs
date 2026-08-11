use domain::model::solution::id::SolutionId;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteSolutionResponse {
    pub solution_id: Uuid,
}

impl From<SolutionId> for DeleteSolutionResponse {
    fn from(value: SolutionId) -> Self {
        Self {
            solution_id: value.into_uuid(),
        }
    }
}
