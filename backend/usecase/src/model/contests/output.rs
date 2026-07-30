use crate::model::contests::ContestListItem;

pub struct ContestListItemOutput {
    pub code: String,
    pub series_code: String,
}

impl From<ContestListItem> for ContestListItemOutput {
    fn from(value: ContestListItem) -> Self {
        Self {
            code: value.code,
            series_code: value.series_code,
        }
    }
}
