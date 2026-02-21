use std::sync::Arc;

use derive_new::new;
use domain::ports::repository::solution::tx::SolutionTxManager;
use uuid::Uuid;

use crate::{
    dto::solution::CreatedCommentView,
    model::solution::{CreatedComment, SolutionError},
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
        user_id: String,
        solution_id: Uuid,
        body_md: String,
    ) -> Result<CreatedCommentView, SolutionError> {
        if body_md.trim().is_empty() {
            return Err(SolutionError::BadRequest(
                "comment body cannot be empty".to_string(),
            ));
        }
        if body_md.chars().count() > 2000 {
            return Err(SolutionError::BadRequest(
                "comment body is too long".to_string(),
            ));
        }

        let exists = self.service.solution_exists(solution_id).await?;
        if !exists {
            return Err(SolutionError::BadRequest("solution not found".to_string()));
        }

        let user_name = self.service.get_user_name_by_id(&user_id).await?;
        let mut uow = self.txm.begin().await?;
        let created = uow
            .comments()
            .create_comment(&user_id, solution_id, &body_md)
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
