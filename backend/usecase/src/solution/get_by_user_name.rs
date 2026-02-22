use std::sync::Arc;

use derive_new::new;

use crate::{
    dto::solution::UserSolutionListItemView,
    model::solution::{SolutionError, SolutionListSort},
    service::solution::SolutionService,
};

#[derive(new)]
pub struct GetSolutionsByUserNameUsecase {
    service: Arc<dyn SolutionService>,
}

impl GetSolutionsByUserNameUsecase {
    pub async fn run(
        &self,
        user_name: String,
        sort: SolutionListSort,
    ) -> Result<Vec<UserSolutionListItemView>, SolutionError> {
        let normalized = user_name.trim();
        if normalized.is_empty() {
            return Err(SolutionError::BadRequest(
                "user_name cannot be empty".to_string(),
            ));
        }

        let exists = self.service.user_name_exists(normalized).await?;
        if !exists {
            return Err(SolutionError::NotFound("user not found".to_string()));
        }

        let items = self
            .service
            .get_solutions_by_user_name(normalized.to_string(), sort)
            .await?;
        Ok(items
            .into_iter()
            .map(UserSolutionListItemView::from)
            .collect())
    }
}
