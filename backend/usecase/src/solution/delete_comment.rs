use std::sync::Arc;

use derive_new::new;
use domain::model::user::id::UserId;
use domain::ports::repository::solution::SolutionRepository;
use uuid::Uuid;

use crate::{error::SolutionError, solution::authorization::ensure_comment_owner};

#[derive(new)]
pub struct DeleteCommentUsecase {
    repository: Arc<dyn SolutionRepository>,
}

impl DeleteCommentUsecase {
    pub async fn run(&self, user_id: UserId, comment_id: Uuid) -> Result<Uuid, SolutionError> {
        ensure_comment_owner(
            self.repository.as_ref(),
            comment_id,
            &user_id,
            "you cannot delete this comment",
        )
        .await?;

        self.repository.delete_comment(comment_id).await?;
        Ok(comment_id)
    }
}
