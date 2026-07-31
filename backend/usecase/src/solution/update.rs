use std::sync::Arc;

use derive_new::new;
use domain::ports::repository::solution::{SolutionRepository, tx::SolutionTxManager};
use uuid::Uuid;
use validator::Validate;

use crate::{
    error::SolutionError,
    model::solution::{tags::normalize_tags, update::UpdateSolutionInput},
    solution::authorization::ensure_solution_owner,
};

#[derive(new)]
pub struct UpdateSolutionUsecase {
    txm: Arc<dyn SolutionTxManager>,
    repository: Arc<dyn SolutionRepository>,
}

impl UpdateSolutionUsecase {
    pub async fn run(&self, input: UpdateSolutionInput) -> Result<Uuid, SolutionError> {
        input
            .validate()
            .map_err(|e| SolutionError::BadRequest(e.to_string()))?;

        ensure_solution_owner(
            self.repository.as_ref(),
            input.solution_id,
            &input.user_id,
            "you cannot update this solution",
        )
        .await?;

        let tags = normalize_tags(&input.tags)?;

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
