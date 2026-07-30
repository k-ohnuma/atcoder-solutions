use std::sync::Arc;

use derive_new::new;
use domain::ports::repository::solution::{SolutionRepository, tx::SolutionTxManager};
use validator::Validate;

use crate::{
    error::SolutionError,
    model::solution::{SolutionComment, output::CommentOutput, update_comment::UpdateCommentInput},
    solution::authorization::ensure_comment_owner,
};

#[derive(new)]
pub struct UpdateCommentUsecase {
    txm: Arc<dyn SolutionTxManager>,
    repository: Arc<dyn SolutionRepository>,
}

impl UpdateCommentUsecase {
    pub async fn run(&self, input: UpdateCommentInput) -> Result<CommentOutput, SolutionError> {
        input
            .validate()
            .map_err(|e| SolutionError::BadRequest(e.to_string()))?;

        ensure_comment_owner(
            self.repository.as_ref(),
            input.comment_id,
            &input.user_id,
            "you cannot update this comment",
        )
        .await?;

        let user_name = self.repository.get_user_name_by_id(&input.user_id).await?;
        let mut uow = self.txm.begin().await?;
        let updated = uow
            .comments()
            .update_comment(input.comment_id, &input.body_md)
            .await?;
        uow.commit().await?;

        Ok(CommentOutput::from(SolutionComment {
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
