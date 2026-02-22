use std::sync::Arc;

use derive_new::new;
use domain::ports::repository::solution::tx::SolutionTxManager;
use uuid::Uuid;
use validator::Validate;

use crate::{
    model::solution::{SolutionError, update::UpdateSolutionInput},
    service::solution::SolutionService,
};

#[derive(new)]
pub struct UpdateSolutionUsecase {
    txm: Arc<dyn SolutionTxManager>,
    service: Arc<dyn SolutionService>,
}

impl UpdateSolutionUsecase {
    pub async fn run(&self, input: UpdateSolutionInput) -> Result<Uuid, SolutionError> {
        input
            .validate()
            .map_err(|e| SolutionError::BadRequest(e.to_string()))?;

        let exists = self.service.solution_exists(input.solution_id).await?;
        if !exists {
            return Err(SolutionError::NotFound("solution not found".to_string()));
        }

        let owner_user_id = self.service.get_solution_user_id(input.solution_id).await?;
        if owner_user_id != input.user_id {
            return Err(SolutionError::Forbidden(
                "you cannot update this solution".to_string(),
            ));
        }

        let tags = input
            .tags
            .into_iter()
            .filter(|e| !e.trim().is_empty())
            .collect::<Vec<_>>();
        if tags.iter().any(|tag| tag.chars().count() > 24) {
            return Err(SolutionError::BadRequest("tag is too long".to_string()));
        }

        let mut uow = self.txm.begin().await?;
        uow.solutions()
            .update(
                input.solution_id,
                &input.title,
                &input.body_md,
                &input.submit_url,
            )
            .await?;

        let tag_ids = uow.tags().upsert(&tags).await?;
        uow.solutions()
            .replace_tags(input.solution_id, &tag_ids)
            .await?;
        uow.commit().await?;

        Ok(input.solution_id)
    }
}
