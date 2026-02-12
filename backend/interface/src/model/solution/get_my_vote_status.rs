use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMyVoteStatusRequest {
    pub solution_id: Uuid,
}

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
