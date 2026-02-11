use crate::model::contests::ContestListItem;

pub struct ContestListItemView {
    pub code: String,
    pub series_code: String,
}

impl From<ContestListItem> for ContestListItemView {
    fn from(value: ContestListItem) -> Self {
        Self {
            code: value.code,
            series_code: value.series_code,
        }
    }
}
