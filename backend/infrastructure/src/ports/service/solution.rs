use async_trait::async_trait;
use derive_new::new;
use domain::error::repository::RepositoryError;
use usecase::{
    model::solution::{SolutionDetails, SolutionListItem},
    service::solution::SolutionService,
};
use uuid::Uuid;

use crate::error::map_sqlx_error;
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
    ) -> Result<Vec<SolutionListItem>, RepositoryError> {
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
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(map_sqlx_error)?;

        Ok(solutions.into_iter().map(SolutionListItem::from).collect())
    }
    async fn get_solution_by_solution_id(
        &self,
        solution_id: Uuid,
    ) -> Result<SolutionDetails, RepositoryError> {
        let solution = sqlx::query_as!(
            SolutionDetails,
            r#"
                SELECT s.id, s.title, s.problem_id, p.title as problem_title, s.user_id, u.user_name, s.body_md, s.submit_url, s.created_at, s.updated_at
                FROM solutions s
                JOIN users u on s.user_id = u.id
                JOIN problems p on s.problem_id = p.id
                WHERE s.id = $1
            "#,
            solution_id
        )
        .fetch_one(self.db.inner_ref())
        .await
        .map_err(map_sqlx_error)?;

        Ok(solution)
    }
}
