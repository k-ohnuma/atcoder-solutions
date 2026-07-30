use std::sync::Arc;

use derive_new::new;
use domain::ports::repository::solution::{SolutionRepository, tx::SolutionTxManager};
use validator::Validate;

use crate::{
    error::SolutionError,
    model::solution::{
        CreatedComment, create_comment::CreateCommentInput, output::CreatedCommentOutput,
    },
};

#[derive(new)]
pub struct CreateCommentUsecase {
    txm: Arc<dyn SolutionTxManager>,
    repository: Arc<dyn SolutionRepository>,
}

impl CreateCommentUsecase {
    pub async fn run(
        &self,
        input: CreateCommentInput,
    ) -> Result<CreatedCommentOutput, SolutionError> {
        input
            .validate()
            .map_err(|e| SolutionError::BadRequest(e.to_string()))?;

        let exists = self.repository.solution_exists(input.solution_id).await?;
        if !exists {
            return Err(SolutionError::BadRequest("solution not found".to_string()));
        }

        let user_name = self.repository.get_user_name_by_id(&input.user_id).await?;
        let mut uow = self.txm.begin().await?;
        let created = uow
            .comments()
            .create_comment(&input.user_id, input.solution_id, &input.body_md)
            .await?;
        uow.commit().await?;

        Ok(CreatedCommentOutput::from(CreatedComment {
            id: created.id,
            user_id: created.user_id,
            user_name,
            solution_id: created.solution_id,
            body_md: created.body_md,
            created_at: created.created_at,
            updated_at: created.updated_at,
        }))
    }
}
