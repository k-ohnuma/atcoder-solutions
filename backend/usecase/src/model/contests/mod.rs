use domain::model::problem::ContestSeries;

use crate::error::ContestError;

pub mod output;

pub use domain::read_model::contests::ContestListItem;

#[derive(Clone, Copy)]
pub struct GetContestsBySeriesInput {
    pub series: ContestSeries,
}

impl GetContestsBySeriesInput {
    pub fn try_new(series: String) -> Result<Self, ContestError> {
        let series =
            ContestSeries::try_from(series).map_err(|e| ContestError::BadRequest(e.msg()))?;
        Ok(Self { series })
    }
}

#[cfg(test)]
mod tests {
    use super::{ContestError, GetContestsBySeriesInput};

    #[test]
    fn contest_series_input_accepts_supported_series() {
        assert!(GetContestsBySeriesInput::try_new("abc".to_string()).is_ok());
    }

    #[test]
    fn contest_series_input_rejects_invalid_series() {
        assert!(matches!(
            GetContestsBySeriesInput::try_new("invalid".to_string()),
            Err(ContestError::BadRequest(message)) if message == "invalid"
        ));
    }
}
