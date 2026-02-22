use std::sync::Arc;

use derive_new::new;
use domain::ports::repository::solution::tx::SolutionTxManager;
use uuid::Uuid;

use crate::{model::solution::SolutionError, service::solution::SolutionService};

#[derive(new)]
pub struct DeleteCommentUsecase {
    txm: Arc<dyn SolutionTxManager>,
    service: Arc<dyn SolutionService>,
}

impl DeleteCommentUsecase {
    pub async fn run(&self, user_id: String, comment_id: Uuid) -> Result<Uuid, SolutionError> {
        let exists = self.service.comment_exists(comment_id).await?;
        if !exists {
            return Err(SolutionError::NotFound("comment not found".to_string()));
        }

        let owner_user_id = self.service.get_comment_user_id(comment_id).await?;
        if owner_user_id != user_id {
            return Err(SolutionError::Forbidden(
                "you cannot delete this comment".to_string(),
            ));
        }

        let mut uow = self.txm.begin().await?;
        uow.comments().delete_comment(comment_id).await?;
        uow.commit().await?;
        Ok(comment_id)
    }
}
