use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMyVoteStatusResponse {
    pub solution_id: Uuid,
    pub liked: bool,
}

impl GetMyVoteStatusResponse {
    pub fn new(solution_id: Uuid, liked: bool) -> Self {
        Self { solution_id, liked }
    }
}
