use async_trait::async_trait;
use derive_new::new;
use domain::{
    error::repository::RepositoryError,
    ports::repository::problem::tx::{ProblemRepositoryTx, ProblemTxManager, ProblemUnitOfWork},
};
use sqlx::{Postgres, Transaction};

use crate::{database::ConnectionPool, error::map_sqlx_error};

#[derive(new)]
pub struct ProblemTransactionManager {
    db: ConnectionPool,
}

#[async_trait]
impl ProblemTxManager for ProblemTransactionManager {
    async fn begin(&self) -> Result<Box<dyn ProblemUnitOfWork>, RepositoryError> {
        let tx = self.db.inner_ref().begin().await.map_err(map_sqlx_error)?;
        Ok(Box::new(ProblemUnitOfWorkImpl::new(tx)))
    }
}

pub struct ProblemUnitOfWorkImpl {
    tx: Transaction<'static, Postgres>,
}

impl ProblemUnitOfWorkImpl {
    fn new(tx: Transaction<'static, Postgres>) -> Self {
        Self { tx }
    }
    fn conn(&mut self) -> &mut sqlx::PgConnection {
        self.tx.as_mut()
    }
}

#[async_trait]
impl ProblemUnitOfWork for ProblemUnitOfWorkImpl {
    fn problems(&mut self) -> &mut dyn ProblemRepositoryTx {
        self
    }

    async fn commit(mut self: Box<Self>) -> Result<(), RepositoryError> {
        self.tx.commit().await.map_err(map_sqlx_error)?;
        Ok(())
    }

    async fn rollback(mut self: Box<Self>) -> Result<(), RepositoryError> {
        self.tx.rollback().await.map_err(map_sqlx_error)?;
        Ok(())
    }
}

#[async_trait]
impl ProblemRepositoryTx for ProblemUnitOfWorkImpl {
    async fn upsert_contest(
        &mut self,
        contest_code: &str,
        series_code: &str,
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
        .execute(self.conn())
        .await
        .map_err(map_sqlx_error)?;
        Ok(())
    }

    async fn upsert_problem(
        &mut self,
        problem_id: &str,
        contest_code: &str,
        problem_index: &str,
        title: &str,
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
            problem_id,
            contest_code,
            problem_index,
            title
        )
        .execute(self.conn())
        .await
        .map_err(map_sqlx_error)?;
        Ok(())
    }
}
