pub mod tx;

use async_trait::async_trait;
use derive_new::new;
use domain::{
    error::repository::RepositoryError,
    model::problem::{ContestSeries, Problem},
    ports::repository::problem::ProblemRepository,
};

use crate::database::ConnectionPool;
use crate::error::map_sqlx_error;

#[derive(new)]
pub struct ProblemRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl ProblemRepository for ProblemRepositoryImpl {
    async fn get_problem_ids_with_difficulty(
        &self,
        problem_ids: &[String],
    ) -> Result<Vec<String>, RepositoryError> {
        if problem_ids.is_empty() {
            return Ok(vec![]);
        }

        let rows = sqlx::query_scalar!(
            r#"
            SELECT id
            FROM problems
            WHERE id = ANY($1)
              AND difficulty IS NOT NULL
            "#,
            problem_ids
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(map_sqlx_error)?;

        Ok(rows)
    }

    async fn get_contest_codes_by_series(
        &self,
        series: ContestSeries,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<String>, RepositoryError> {
        let contest_codes = sqlx::query_scalar!(
            r#"
            SELECT c.code
            FROM contests c
            JOIN contest_series s ON s.code = c.series_code
            WHERE s.code = $1
            ORDER BY c.code DESC
            LIMIT $2 OFFSET $3
            "#,
            series.to_string(),
            limit,
            offset,
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(map_sqlx_error)?;

        Ok(contest_codes)
    }

    async fn get_problems_by_contest_codes(
        &self,
        contest_codes: &[String],
    ) -> Result<Vec<Problem>, RepositoryError> {
        if contest_codes.is_empty() {
            return Ok(vec![]);
        }

        let problems = sqlx::query_as!(
            Problem,
            r#"
            SELECT p.id, p.contest_code, p.problem_index, p.title, p.difficulty
            FROM problems p
            WHERE p.contest_code = ANY($1)
            ORDER BY p.contest_code DESC
            "#,
            contest_codes
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(map_sqlx_error)?;

        Ok(problems)
    }

    async fn search_problems_by_contest_series(
        &self,
        series: ContestSeries,
        query: &str,
    ) -> Result<Vec<Problem>, RepositoryError> {
        let query = query.to_lowercase();
        let problems = sqlx::query_as!(
            Problem,
            r#"
            WITH matched_contests AS (
                SELECT c.code
                FROM contests c
                JOIN contest_series s ON s.code = c.series_code
                WHERE s.code = $1
                  AND position($2 in LOWER(c.code)) > 0
            )
            SELECT p.id, p.contest_code, p.problem_index, p.title, p.difficulty
            FROM problems p
            JOIN contests c ON c.code = p.contest_code
            JOIN contest_series s ON s.code = c.series_code
            WHERE s.code = $1
              AND (
                  p.contest_code IN (SELECT code FROM matched_contests)
                  OR position($2 in LOWER(p.id)) > 0
                  OR position($2 in LOWER(p.problem_index)) > 0
                  OR position($2 in LOWER(p.title)) > 0
              )
            ORDER BY p.contest_code DESC
            "#,
            series.to_string(),
            query,
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(map_sqlx_error)?;

        Ok(problems)
    }

    async fn get_problem_by_id(&self, problem_id: &str) -> Result<Problem, RepositoryError> {
        let problem = sqlx::query_as!(
            Problem,
            r#"
            SELECT p.id, p.contest_code, p.problem_index, p.title, p.difficulty
            FROM problems p
            WHERE p.id = $1
            "#,
            problem_id
        )
        .fetch_optional(self.db.inner_ref())
        .await
        .map_err(map_sqlx_error)?
        .ok_or_else(|| RepositoryError::NotFound(problem_id.to_string()))?;

        Ok(problem)
    }

    async fn problem_exists(&self, problem_id: &str) -> Result<bool, RepositoryError> {
        let rec = sqlx::query!(
            r#"
                SELECT EXISTS(
                    SELECT 1
                    FROM problems
                    WHERE id = $1
                ) AS "exists!"
            "#,
            problem_id
        )
        .fetch_one(self.db.inner_ref())
        .await
        .map_err(map_sqlx_error)?;

        Ok(rec.exists)
    }

    async fn get_problems_by_contest(
        &self,
        contest: &str,
    ) -> Result<Vec<Problem>, RepositoryError> {
        let problems: Vec<Problem> = sqlx::query_as!(
            Problem,
            r#"
            SELECT p.id, p.contest_code, p.problem_index, p.title, p.difficulty
            FROM problems p
            JOIN contests c ON c.code = p.contest_code
            JOIN contest_series s ON s.code = c.series_code
            WHERE p.contest_code = $1
            "#,
            contest.to_ascii_lowercase()
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(map_sqlx_error)?;
        Ok(problems)
    }
}
