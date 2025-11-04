use async_trait::async_trait;
use derive_new::new;
use shared::error::repository::RepositoryError;
use usecase::{dto::solution::{SolutionListItemView, SolutionView}, service::solution::SolutionService};
use uuid::Uuid;

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
                SELECT s.id, s.title, s.problem_id, s.user_id, u.user_name, s.created_at, s.updated_at
                FROM solutions s
                JOIN users u on s.user_id = u.id
                WHERE s.problem_id = $1
                ORDER BY s.created_at DESC
            "#,
            problem_id
        ).fetch_all(self.db.inner_ref()).await?;

        Ok(solutions
            .into_iter()
            .map(SolutionListItemView::from)
            .collect())
    }
    async fn get_solution_by_solution_id(
        &self,
        solution_id: Uuid,
    ) -> Result<SolutionView, RepositoryError> {
        let solution = sqlx::query_as!(
            SolutionView,
            r#"
                SELECT s.id, s.title, s.problem_id, p.title as problem_title, s.user_id, u.user_name, s.body_md, s.submit_url, s.created_at, s.updated_at
                FROM solutions s
                JOIN users u on s.user_id = u.id
                JOIN problems p on s.problem_id = p.id
                WHERE s.id = $1
            "#,
            solution_id
        ).fetch_one(self.db.inner_ref()).await?;

        Ok(solution)
    }
}
