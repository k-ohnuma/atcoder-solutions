use std::sync::Arc;

use derive_new::new;

use crate::{
    dto::solution::UserSolutionListItemView,
    model::solution::{SolutionError, list::GetSolutionsByUserNameInput},
    service::solution::SolutionService,
};

#[derive(new)]
pub struct GetSolutionsByUserNameUsecase {
    service: Arc<dyn SolutionService>,
}

impl GetSolutionsByUserNameUsecase {
    pub async fn run(
        &self,
        input: GetSolutionsByUserNameInput,
    ) -> Result<Vec<UserSolutionListItemView>, SolutionError> {
        let exists = self.service.user_name_exists(&input.user_name).await?;
        if !exists {
            return Err(SolutionError::NotFound("user not found".to_string()));
        }

        let items = self
            .service
            .get_solutions_by_user_name(input.user_name, input.sort)
            .await?;
        Ok(items
            .into_iter()
            .map(UserSolutionListItemView::from)
            .collect())
    }
}
