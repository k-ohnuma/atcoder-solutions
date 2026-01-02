// TODO: トランザクションをusecaseに寄せる(日時実行しか使ってないので優先度低)
use std::collections::HashSet;

use async_trait::async_trait;
use derive_new::new;
use domain::{
    model::problem::{ContestSeries, Problem},
    ports::repository::problem::ProblemRepository,
};
use shared::error::repository::RepositoryError;
use sqlx::{Postgres, Transaction};

use crate::database::ConnectionPool;

#[derive(new)]
pub struct ProblemRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl ProblemRepository for ProblemRepositoryImpl {
    async fn create_records(&self, problems: Vec<Problem>) -> Result<(), RepositoryError> {
        let mut tx = self
            .db
            .inner_ref()
            .begin()
            .await
            .map_err(RepositoryError::TransactionError)?;
        {
            let contests = problems
                .iter()
                .filter_map(|e| {
                    let series = ContestSeries::try_from(e.contest_code.as_str()).ok()?;
                    Some((e.contest_code.as_str(), series))
                })
                .collect::<HashSet<(&str, ContestSeries)>>();

            for contest in contests.iter() {
                safe_insert_contest(contest.0, contest.1.into(), &mut tx).await?;
            }
        }

        for problem in problems.iter() {
            upsert_problem(problem, &mut tx).await?;
        }
        tx.commit()
            .await
            .map_err(RepositoryError::TransactionError)?;
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
        .map_err(RepositoryError::from)?;
        Ok(problems)
    }
}

async fn safe_insert_contest(
    contest_code: &str,
    series_code: &str,
    tx: &mut Transaction<'_, Postgres>,
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
    .execute(&mut **tx)
    .await
    .map_err(RepositoryError::from)?;
    Ok(())
}

async fn upsert_problem(
    problem: &Problem,
    tx: &mut Transaction<'_, Postgres>,
) -> Result<(), RepositoryError> {
    // TODO: upsertする必要があるか要確認。
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
    .execute(&mut **tx)
    .await
    .map_err(RepositoryError::from)?;
    Ok(())
}
