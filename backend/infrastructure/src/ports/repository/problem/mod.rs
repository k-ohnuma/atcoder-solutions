pub mod tx;

use async_trait::async_trait;
use derive_new::new;
use domain::{
    error::repository::RepositoryError,
    model::problem::{ContestSeries, Problem, contest_code::ContestCode, id::ProblemId},
    ports::repository::problem::ProblemRepository,
};

use crate::error::map_sqlx_error;
use crate::{database::ConnectionPool, model::problems::ProblemRow};

#[derive(new)]
pub struct ProblemRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl ProblemRepository for ProblemRepositoryImpl {
    async fn get_problem_ids_with_difficulty(
        &self,
        problem_ids: &[ProblemId],
    ) -> Result<Vec<ProblemId>, RepositoryError> {
        if problem_ids.is_empty() {
            return Ok(vec![]);
        }

        let problem_ids = problem_ids
            .iter()
            .map(|problem_id| problem_id.as_str().to_string())
            .collect::<Vec<_>>();
        let rows = sqlx::query_scalar!(
            r#"
            SELECT id
            FROM problems
            WHERE id = ANY($1)
              AND difficulty IS NOT NULL
            "#,
            &problem_ids
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(map_sqlx_error)?;

        rows.into_iter()
            .map(|id| {
                ProblemId::try_new(id)
                    .map_err(|error| RepositoryError::Unexpected(error.to_string()))
            })
            .collect()
    }

    async fn get_contest_codes_by_series(
        &self,
        series: ContestSeries,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ContestCode>, RepositoryError> {
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

        contest_codes
            .into_iter()
            .map(|code| {
                ContestCode::try_new(code)
                    .map_err(|error| RepositoryError::Unexpected(error.to_string()))
            })
            .collect()
    }

    async fn get_problems_by_contest_codes(
        &self,
        contest_codes: &[ContestCode],
    ) -> Result<Vec<Problem>, RepositoryError> {
        if contest_codes.is_empty() {
            return Ok(vec![]);
        }

        let contest_codes = contest_codes
            .iter()
            .map(|code| code.as_str().to_string())
            .collect::<Vec<_>>();
        let problems = sqlx::query_as!(
            ProblemRow,
            r#"
            SELECT p.id, p.contest_code, p.problem_index, p.title, p.difficulty
            FROM problems p
            WHERE p.contest_code = ANY($1)
            ORDER BY p.contest_code DESC
            "#,
            &contest_codes
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(map_sqlx_error)?;

        problems
            .into_iter()
            .map(|problem| {
                Problem::try_from(problem)
                    .map_err(|error| RepositoryError::Unexpected(error.to_string()))
            })
            .collect()
    }

    async fn search_problems_by_contest_series(
        &self,
        series: ContestSeries,
        query: &str,
    ) -> Result<Vec<Problem>, RepositoryError> {
        let query = query.to_lowercase();
        let problems = sqlx::query_as!(
            ProblemRow,
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

        problems
            .into_iter()
            .map(|problem| {
                Problem::try_from(problem)
                    .map_err(|error| RepositoryError::Unexpected(error.to_string()))
            })
            .collect()
    }

    async fn get_problem_by_id(&self, problem_id: &ProblemId) -> Result<Problem, RepositoryError> {
        let problem_id = problem_id.as_str();
        let problem = sqlx::query_as!(
            ProblemRow,
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

        Problem::try_from(problem).map_err(|error| RepositoryError::Unexpected(error.to_string()))
    }

    async fn problem_exists(&self, problem_id: &ProblemId) -> Result<bool, RepositoryError> {
        let problem_id = problem_id.as_str();
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
        contest: &ContestCode,
    ) -> Result<Vec<Problem>, RepositoryError> {
        let problems = sqlx::query_as!(
            ProblemRow,
            r#"
            SELECT p.id, p.contest_code, p.problem_index, p.title, p.difficulty
            FROM problems p
            JOIN contests c ON c.code = p.contest_code
            JOIN contest_series s ON s.code = c.series_code
            WHERE p.contest_code = $1
            "#,
            contest.as_str().to_ascii_lowercase()
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(map_sqlx_error)?;
        problems
            .into_iter()
            .map(|problem| {
                Problem::try_from(problem)
                    .map_err(|error| RepositoryError::Unexpected(error.to_string()))
            })
            .collect()
    }
}
