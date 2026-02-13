use async_trait::async_trait;
use derive_new::new;
use domain::error::repository::RepositoryError;
use usecase::{
    model::solution::{SolutionComment, SolutionDetails, SolutionListItem, SolutionListSort},
    service::solution::SolutionService,
};
use uuid::Uuid;

use crate::error::map_sqlx_error;
use crate::{
    database::ConnectionPool,
    model::solution::{SolutionCommentViewRaw, SolutionListItemViewRaw},
};

#[derive(new)]
pub struct SolutionServiceImpl {
    db: ConnectionPool,
}

#[async_trait]
impl SolutionService for SolutionServiceImpl {
    async fn get_solutions_by_problem_id(
        &self,
        problem_id: String,
        sort: SolutionListSort,
    ) -> Result<Vec<SolutionListItem>, RepositoryError> {
        let problem_id_ref = problem_id.as_str();
        let solutions = match sort {
            SolutionListSort::Latest => {
                sqlx::query_as!(
                    SolutionListItemViewRaw,
                    r#"
                        SELECT s.id, s.title, s.problem_id, s.user_id, u.user_name,
                               COUNT(sv.user_id) AS "votes_count!",
                               s.created_at, s.updated_at
                        FROM solutions s
                        JOIN users u on s.user_id = u.id
                        LEFT JOIN solution_votes sv ON sv.solution_id = s.id
                        WHERE s.problem_id = $1
                        GROUP BY s.id, s.title, s.problem_id, s.user_id, u.user_name, s.created_at, s.updated_at
                        ORDER BY s.created_at DESC
                    "#,
                    problem_id_ref
                )
                .fetch_all(self.db.inner_ref())
                .await
                .map_err(map_sqlx_error)?
            }
            SolutionListSort::Votes => {
                sqlx::query_as!(
                    SolutionListItemViewRaw,
                    r#"
                        SELECT s.id, s.title, s.problem_id, s.user_id, u.user_name,
                               COUNT(sv.user_id) AS "votes_count!",
                               s.created_at, s.updated_at
                        FROM solutions s
                        JOIN users u on s.user_id = u.id
                        LEFT JOIN solution_votes sv ON sv.solution_id = s.id
                        WHERE s.problem_id = $1
                        GROUP BY s.id, s.title, s.problem_id, s.user_id, u.user_name, s.created_at, s.updated_at
                        ORDER BY "votes_count!" DESC, s.created_at DESC
                    "#,
                    problem_id_ref
                )
                .fetch_all(self.db.inner_ref())
                .await
                .map_err(map_sqlx_error)?
            }
        };

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

    async fn get_solution_votes_count(&self, solution_id: Uuid) -> Result<i64, RepositoryError> {
        let rec = sqlx::query!(
            r#"
                SELECT COUNT(*) AS "count!" FROM solution_votes
                WHERE solution_id = $1
            "#,
            solution_id
        )
        .fetch_one(self.db.inner_ref())
        .await
        .map_err(map_sqlx_error)?;

        Ok(rec.count)
    }

    async fn has_user_voted_solution(
        &self,
        user_id: String,
        solution_id: Uuid,
    ) -> Result<bool, RepositoryError> {
        let rec = sqlx::query!(
            r#"
                SELECT EXISTS(
                    SELECT 1
                    FROM solution_votes
                    WHERE user_id = $1 AND solution_id = $2
                ) AS "exists!"
            "#,
            user_id,
            solution_id
        )
        .fetch_one(self.db.inner_ref())
        .await
        .map_err(map_sqlx_error)?;

        Ok(rec.exists)
    }

    async fn solution_exists(&self, solution_id: Uuid) -> Result<bool, RepositoryError> {
        let rec = sqlx::query!(
            r#"
                SELECT EXISTS(
                    SELECT 1
                    FROM solutions
                    WHERE id = $1
                ) AS "exists!"
            "#,
            solution_id
        )
        .fetch_one(self.db.inner_ref())
        .await
        .map_err(map_sqlx_error)?;

        Ok(rec.exists)
    }

    async fn get_comments_by_solution_id(
        &self,
        solution_id: Uuid,
    ) -> Result<Vec<SolutionComment>, RepositoryError> {
        let comments = sqlx::query_as!(
            SolutionCommentViewRaw,
            r#"
                SELECT c.id, c.user_id, u.user_name, c.solution_id, c.body_md, c.created_at, c.updated_at
                FROM comments c
                JOIN users u ON c.user_id = u.id
                WHERE c.solution_id = $1
                ORDER BY c.created_at ASC
            "#,
            solution_id
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(map_sqlx_error)?;

        Ok(comments.into_iter().map(SolutionComment::from).collect())
    }

    async fn get_user_name_by_id(&self, user_id: &str) -> Result<String, RepositoryError> {
        let rec = sqlx::query!(
            r#"
                SELECT user_name
                FROM users
                WHERE id = $1
            "#,
            user_id
        )
        .fetch_one(self.db.inner_ref())
        .await
        .map_err(map_sqlx_error)?;

        Ok(rec.user_name)
    }
}
