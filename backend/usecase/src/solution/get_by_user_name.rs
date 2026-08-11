use std::sync::Arc;

use derive_new::new;
use domain::ports::repository::{solution::SolutionRepository, user::UserRepository};

use crate::{
    error::SolutionError,
    model::solution::{list::GetSolutionsByUserNameInput, output::UserSolutionListItemOutput},
};

#[derive(new)]
pub struct GetSolutionsByUserNameUsecase {
    solution_repository: Arc<dyn SolutionRepository>,
    user_repository: Arc<dyn UserRepository>,
}

impl GetSolutionsByUserNameUsecase {
    pub async fn run(
        &self,
        input: GetSolutionsByUserNameInput,
    ) -> Result<Vec<UserSolutionListItemOutput>, SolutionError> {
        let exists = self
            .user_repository
            .user_name_exists(&input.user_name)
            .await?;
        if !exists {
            return Err(SolutionError::NotFound("user not found".to_string()));
        }

        let items = self
            .solution_repository
            .get_solutions_by_user_name(input.user_name, input.sort)
            .await?;
        Ok(items
            .into_iter()
            .map(UserSolutionListItemOutput::from)
            .collect())
    }
}
