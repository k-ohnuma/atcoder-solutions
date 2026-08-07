use chrono::{DateTime, Utc};
use domain::{
    error::repository::RepositoryError,
    model::problem::{ContestSeries, contest_code::ContestCode},
    read_model::contests::ContestListItem,
};
use uuid::Uuid;

pub struct ContestListItemViewRaw {
    pub id: Uuid,
    pub code: String,
    pub series_code: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TryFrom<ContestListItemViewRaw> for ContestListItem {
    type Error = RepositoryError;

    fn try_from(value: ContestListItemViewRaw) -> Result<Self, Self::Error> {
        Ok(Self {
            code: ContestCode::try_new(value.code)
                .map_err(|error| RepositoryError::Unexpected(error.to_string()))?,
            series: ContestSeries::try_from(value.series_code)
                .map_err(|error| RepositoryError::Unexpected(error.to_string()))?,
        })
    }
}
