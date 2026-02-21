use std::sync::Arc;

use derive_new::new;
use uuid::Uuid;

use crate::{
    dto::solution::SolutionCommentView, model::solution::SolutionError,
    service::solution::SolutionService,
};

#[derive(new)]
pub struct GetCommentsBySolutionIdUsecase {
    service: Arc<dyn SolutionService>,
}

impl GetCommentsBySolutionIdUsecase {
    pub async fn run(&self, solution_id: Uuid) -> Result<Vec<SolutionCommentView>, SolutionError> {
        let exists = self.service.solution_exists(solution_id).await?;
        if !exists {
            return Err(SolutionError::NotFound("solution not found".to_string()));
        }
        let comments = self
            .service
            .get_comments_by_solution_id(solution_id)
            .await?;
        Ok(comments
            .into_iter()
            .map(SolutionCommentView::from)
            .collect())
    }
}
