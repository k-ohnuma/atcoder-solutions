use domain::error::repository::RepositoryError;
use domain::model::problem::ContestSeries;
use thiserror::Error;

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

#[derive(Debug, Clone)]
pub struct ContestListItem {
    pub code: String,
    pub series_code: String,
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

#[derive(Debug, Error)]
pub enum ContestError {
    #[error("{0}")]
    BadRequest(String),
    #[error("{0}")]
    DBError(String),
    #[error("{0}")]
    NotFound(String),
}

impl From<RepositoryError> for ContestError {
    fn from(value: RepositoryError) -> Self {
        match value {
            RepositoryError::NotFound(msg) => ContestError::NotFound(msg),
            RepositoryError::TransactionError(msg) => ContestError::DBError(msg.to_string()),
            RepositoryError::UniqueViolation(msg) => ContestError::DBError(msg),
            RepositoryError::ForeignKeyViolation(msg) => ContestError::DBError(msg.to_string()),
            RepositoryError::NotNullViolation(msg) => ContestError::DBError(msg.to_string()),
            RepositoryError::CheckViolation(msg) => ContestError::DBError(msg.to_string()),
            RepositoryError::Connection(msg) => ContestError::DBError(msg.to_string()),
            RepositoryError::Query(msg) => ContestError::DBError(msg.to_string()),
            RepositoryError::Unexpected(msg) => ContestError::DBError(msg.to_string()),
        }
    }
}
