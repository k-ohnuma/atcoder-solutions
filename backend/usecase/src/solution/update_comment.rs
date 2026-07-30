use std::sync::Arc;

use derive_new::new;
use domain::ports::repository::solution::{SolutionRepository, tx::SolutionTxManager};
use validator::Validate;

use crate::{
    dto::solution::CreatedCommentView,
    error::SolutionError,
    model::solution::{CreatedComment, update_comment::UpdateCommentInput},
};

#[derive(new)]
pub struct UpdateCommentUsecase {
    txm: Arc<dyn SolutionTxManager>,
    repository: Arc<dyn SolutionRepository>,
}

impl UpdateCommentUsecase {
    pub async fn run(
        &self,
        input: UpdateCommentInput,
    ) -> Result<CreatedCommentView, SolutionError> {
        input
            .validate()
            .map_err(|e| SolutionError::BadRequest(e.to_string()))?;

        let exists = self.repository.comment_exists(input.comment_id).await?;
        if !exists {
            return Err(SolutionError::NotFound("comment not found".to_string()));
        }

        let owner_user_id = self
            .repository
            .get_comment_user_id(input.comment_id)
            .await?;
        if owner_user_id != input.user_id {
            return Err(SolutionError::Forbidden(
                "you cannot update this comment".to_string(),
            ));
        }

        let user_name = self.repository.get_user_name_by_id(&input.user_id).await?;
        let mut uow = self.txm.begin().await?;
        let updated = uow
            .comments()
            .update_comment(input.comment_id, &input.body_md)
            .await?;
        uow.commit().await?;

        Ok(CreatedCommentView::from(CreatedComment {
            id: updated.id,
            user_id: updated.user_id,
            user_name,
            solution_id: updated.solution_id,
            body_md: updated.body_md,
            created_at: updated.created_at,
            updated_at: updated.updated_at,
        }))
    }
}
