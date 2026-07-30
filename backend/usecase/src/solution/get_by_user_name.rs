use std::sync::Arc;

use derive_new::new;
use domain::ports::repository::solution::SolutionRepository;

use crate::{
    dto::solution::UserSolutionListItemView, error::SolutionError,
    model::solution::list::GetSolutionsByUserNameInput,
};

#[derive(new)]
pub struct GetSolutionsByUserNameUsecase {
    repository: Arc<dyn SolutionRepository>,
}

impl GetSolutionsByUserNameUsecase {
    pub async fn run(
        &self,
        input: GetSolutionsByUserNameInput,
    ) -> Result<Vec<UserSolutionListItemView>, SolutionError> {
        let exists = self.repository.user_name_exists(&input.user_name).await?;
        if !exists {
            return Err(SolutionError::NotFound("user not found".to_string()));
        }

        let items = self
            .repository
            .get_solutions_by_user_name(input.user_name, input.sort)
            .await?;
        Ok(items
            .into_iter()
            .map(UserSolutionListItemView::from)
            .collect())
    }
}
