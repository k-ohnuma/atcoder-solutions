pub mod tx;

use std::collections::HashSet;

use async_trait::async_trait;
use derive_new::new;
use domain::{
    error::repository::RepositoryError,
    model::problem::{ContestSeries, Problem},
    ports::repository::problem::ProblemRepository,
};
use sqlx::PgPool;

use crate::database::ConnectionPool;
use crate::error::map_sqlx_error;

#[derive(new)]
pub struct ProblemRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl ProblemRepository for ProblemRepositoryImpl {
    async fn create_records(&self, problems: Vec<Problem>) -> Result<(), RepositoryError> {
        let contests = problems
            .iter()
            .map(|e| {
                let series =
                    ContestSeries::try_from(e.contest_code.as_str()).unwrap_or(ContestSeries::OTHER);
                (e.contest_code.as_str(), series)
            })
            .collect::<HashSet<(&str, ContestSeries)>>();

        for contest in contests.iter() {
            safe_insert_contest(contest.0, contest.1.into(), self.db.inner_ref()).await?;
        }

        for problem in problems.iter() {
            upsert_problem(problem, self.db.inner_ref()).await?;
        }
        Ok(())
    }
    async fn get_problems_by_contest_series(
        &self,
        series: ContestSeries,
    ) -> Result<Vec<Problem>, RepositoryError> {
        let problems: Vec<Problem> = sqlx::query_as!(
            Problem,
            r#"
            SELECT p.id, p.contest_code, p.problem_index, p.title
            FROM problems p
            JOIN contests c ON c.code = p.contest_code
            JOIN contest_series s ON s.code = c.series_code
            WHERE s.code = $1
            ORDER BY p.contest_code DESC
            "#,
            series.to_string(),
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(map_sqlx_error)?;
        Ok(problems)
    }
    async fn get_problems_by_contest(
        &self,
        contest: &str,
    ) -> Result<Vec<Problem>, RepositoryError> {
        let problems: Vec<Problem> = sqlx::query_as!(
            Problem,
            r#"
            SELECT p.id, p.contest_code, p.problem_index, p.title
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

async fn safe_insert_contest(
    contest_code: &str,
    series_code: &str,
    db: &PgPool,
) -> Result<(), RepositoryError> {
    sqlx::query!(
        r#"
        INSERT INTO contests (code, series_code)
        VALUES ($1, $2)
        ON CONFLICT (code) DO NOTHING
        "#,
        contest_code,
        series_code
    )
    .execute(db)
    .await
    .map_err(map_sqlx_error)?;
    Ok(())
}

async fn upsert_problem(
    problem: &Problem,
    db: &PgPool,
) -> Result<(), RepositoryError> {
    sqlx::query!(
        r#"
        INSERT INTO problems (id, contest_code, problem_index, title)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (id) DO UPDATE
        SET
            contest_code = EXCLUDED.contest_code,
            problem_index = EXCLUDED.problem_index,
            title = EXCLUDED.title
        "#,
        problem.id,
        problem.contest_code,
        problem.problem_index,
        problem.title
    )
    .execute(db)
    .await
    .map_err(map_sqlx_error)?;
    Ok(())
}
