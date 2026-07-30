use std::sync::Arc;

use derive_new::new;
use domain::ports::repository::solution::{SolutionRepository, tx::SolutionTxManager};
use uuid::Uuid;

use crate::{error::SolutionError, solution::authorization::ensure_comment_owner};

#[derive(new)]
pub struct DeleteCommentUsecase {
    txm: Arc<dyn SolutionTxManager>,
    repository: Arc<dyn SolutionRepository>,
}

impl DeleteCommentUsecase {
    pub async fn run(&self, user_id: String, comment_id: Uuid) -> Result<Uuid, SolutionError> {
        ensure_comment_owner(
            self.repository.as_ref(),
            comment_id,
            &user_id,
            "you cannot delete this comment",
        )
        .await?;

        let mut uow = self.txm.begin().await?;
        uow.comments().delete_comment(comment_id).await?;
        uow.commit().await?;
        Ok(comment_id)
    }
}
