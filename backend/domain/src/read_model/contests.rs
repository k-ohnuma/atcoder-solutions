use crate::model::problem::{ContestSeries, contest_code::ContestCode};

#[derive(Debug, Clone)]
pub struct ContestListItem {
    pub code: ContestCode,
    pub series: ContestSeries,
}
