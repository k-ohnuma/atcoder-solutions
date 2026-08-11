use domain::{model::user::name::UserName, read_model::solution::SolutionListSort};

use crate::error::SolutionError;

use super::by_problem::parse_solution_list_sort;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetSolutionsByUserNameInput {
    pub user_name: UserName,
    pub sort: SolutionListSort,
}

impl GetSolutionsByUserNameInput {
    pub fn try_new(user_name: String, sort_by: Option<String>) -> Result<Self, SolutionError> {
        Ok(Self {
            user_name: UserName::try_new(user_name)?,
            sort: parse_solution_list_sort(sort_by)?,
        })
    }
}
