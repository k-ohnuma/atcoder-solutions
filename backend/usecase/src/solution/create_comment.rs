use std::sync::Arc;

use derive_new::new;
use domain::ports::repository::solution::tx::SolutionTxManager;
use validator::Validate;

use crate::{
    dto::solution::CreatedCommentView,
    model::solution::{CreatedComment, SolutionError, create_comment::CreateCommentInput},
    service::solution::SolutionService,
};

#[derive(new)]
pub struct CreateCommentUsecase {
    txm: Arc<dyn SolutionTxManager>,
    service: Arc<dyn SolutionService>,
}

impl CreateCommentUsecase {
    pub async fn run(
        &self,
        input: CreateCommentInput,
    ) -> Result<CreatedCommentView, SolutionError> {
        input
            .validate()
            .map_err(|e| SolutionError::BadRequest(e.to_string()))?;

        let exists = self.service.solution_exists(input.solution_id).await?;
        if !exists {
            return Err(SolutionError::BadRequest("solution not found".to_string()));
        }

        let user_name = self.service.get_user_name_by_id(&input.user_id).await?;
        let mut uow = self.txm.begin().await?;
        let created = uow
            .comments()
            .create_comment(&input.user_id, input.solution_id, &input.body_md)
            .await?;
        uow.commit().await?;

        Ok(CreatedCommentView::from(CreatedComment {
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
