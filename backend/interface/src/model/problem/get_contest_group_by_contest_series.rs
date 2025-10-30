use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetContestGroupByContestSeriesRequestParams {
    pub series: String,
}

