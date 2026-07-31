use crate::error::SolutionError;

use super::validate_limit;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetLatestSolutionsInput {
    pub limit: Option<i32>,
}

impl GetLatestSolutionsInput {
    pub fn try_new(sort_by: Option<String>, limit: Option<i32>) -> Result<Self, SolutionError> {
        if matches!(sort_by.as_deref(), Some(v) if v != "latest") {
            return Err(SolutionError::BadRequest(
                "sort_by must be latest".to_string(),
            ));
        }
        Ok(Self {
            limit: validate_limit(limit)?,
        })
    }
}
