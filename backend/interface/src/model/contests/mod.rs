use serde::Serialize;
use usecase::model::contests::output::ContestListItemOutput;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContestResponse {
    pub code: String,
    pub series_code: String,
}

impl From<ContestListItemOutput> for ContestResponse {
    fn from(value: ContestListItemOutput) -> Self {
        Self {
            code: value.code.into_string(),
            series_code: value.series.into(),
        }
    }
}
