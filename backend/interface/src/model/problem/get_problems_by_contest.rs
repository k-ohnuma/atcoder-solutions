use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetProblemsByContestRequestParams {
    pub contest: String,
}
