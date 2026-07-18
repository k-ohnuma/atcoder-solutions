use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSolutionVotesCountResponse {
    pub solution_id: Uuid,
    pub votes_count: i64,
}

impl GetSolutionVotesCountResponse {
    pub fn new(solution_id: Uuid, votes_count: i64) -> Self {
        Self {
            solution_id,
            votes_count,
        }
    }
}
