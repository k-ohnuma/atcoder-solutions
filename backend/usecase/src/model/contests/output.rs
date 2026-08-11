use domain::{
    model::problem::{ContestSeries, contest_code::ContestCode},
    read_model::contests::ContestListItem,
};

pub struct ContestListItemOutput {
    pub code: ContestCode,
    pub series: ContestSeries,
}

impl From<ContestListItem> for ContestListItemOutput {
    fn from(value: ContestListItem) -> Self {
        Self {
            code: value.code,
            series: value.series,
        }
    }
}
