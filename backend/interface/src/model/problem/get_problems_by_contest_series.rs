use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetProblemsByContestSeriesRequestParams {
    pub series: String
}


