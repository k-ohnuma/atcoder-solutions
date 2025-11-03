use async_trait::async_trait;
use derive_new::new;
use shared::error::repository::RepositoryError;
use usecase::{dto::solution::SolutionListItemView, service::solution::SolutionService};

use crate::{database::ConnectionPool, model::solution::SolutionListItemViewRaw};

#[derive(new)]
pub struct SolutionServiceImpl {
    db: ConnectionPool,
}

#[async_trait]
impl SolutionService for SolutionServiceImpl {
    async fn get_solutions_by_problem_id(
        &self,
        problem_id: String,
    ) -> Result<Vec<SolutionListItemView>, RepositoryError> {
        let solutions = sqlx::query_as!(
            SolutionListItemViewRaw,
            r#"
                SELECT s.id, s.problem_id, s.user_id, u.user_name, s.created_at, s.updated_at
                FROM solutions s
                JOIN users u on s.user_id = u.id
                WHERE s.problem_id = $1
                ORDER BY s.created_at DESC
            "#,
            problem_id
        ).fetch_all(self.db.inner_ref()).await?;

        Ok(solutions.into_iter().map(SolutionListItemView::from).collect())
    }
}
