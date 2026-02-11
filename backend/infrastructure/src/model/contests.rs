use chrono::{DateTime, Utc};
use usecase::model::contests::ContestListItem;
use uuid::Uuid;

pub struct ContestListItemViewRaw {
    pub id: Uuid,
    pub code: String,
    pub series_code: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<ContestListItemViewRaw> for ContestListItem {
    fn from(value: ContestListItemViewRaw) -> Self {
        Self {
            code: value.code,
            series_code: value.series_code,
        }
    }
}
