use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetContestsBySeriesRequestParams {
    pub series: String,
}
