use serde::Serialize;
use usecase::dto::contests::ContestListItemView;

pub mod get_contests_by_series;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContestResponse {
    pub code: String,
    pub series_code: String,
}

impl From<ContestListItemView> for ContestResponse {
    fn from(value: ContestListItemView) -> Self {
        Self {
            code: value.code,
            series_code: value.series_code,
        }
    }
}
