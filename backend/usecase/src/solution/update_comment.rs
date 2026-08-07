use std::sync::Arc;

use derive_new::new;
use domain::ports::repository::solution::SolutionRepository;

use crate::{
    error::SolutionError,
    model::solution::{output::CommentOutput, update_comment::UpdateCommentInput},
    solution::authorization::ensure_comment_owner,
};

#[derive(new)]
pub struct UpdateCommentUsecase {
    repository: Arc<dyn SolutionRepository>,
}

impl UpdateCommentUsecase {
    pub async fn run(&self, input: UpdateCommentInput) -> Result<CommentOutput, SolutionError> {
        ensure_comment_owner(
            self.repository.as_ref(),
            input.comment_id,
            &input.user_id,
            "you cannot update this comment",
        )
        .await?;

        let comment = self
            .repository
            .update_comment(input.comment_id, &input.body)
            .await?;

        Ok(CommentOutput::from(comment))
    }
}
