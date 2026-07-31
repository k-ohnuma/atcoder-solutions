use domain::read_model::solution::SolutionListSort;

use crate::error::SolutionError;

use super::{by_problem::parse_solution_list_sort, normalize_required_string};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetSolutionsByUserNameInput {
    pub user_name: String,
    pub sort: SolutionListSort,
}

impl GetSolutionsByUserNameInput {
    pub fn try_new(user_name: String, sort_by: Option<String>) -> Result<Self, SolutionError> {
        let user_name = normalize_required_string(user_name, "user_name cannot be empty")?;
        Ok(Self {
            user_name,
            sort: parse_solution_list_sort(sort_by)?,
        })
    }
}
