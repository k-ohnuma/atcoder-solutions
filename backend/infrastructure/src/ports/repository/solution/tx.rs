use async_trait::async_trait;
use derive_new::new;
use domain::model::solution::Solution;
use domain::ports::repository::solution::tx::{
    SolutionRespositoryTx, SolutionTxManager, TagRepositoryTx, UnitOfWork,
};
use shared::error::repository::RepositoryError;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

use crate::database::ConnectionPool;

#[derive(new)]
pub struct SolutionTransactionManager {
    db: ConnectionPool,
}

#[async_trait]
impl SolutionTxManager for SolutionTransactionManager {
    async fn begin(&self) -> Result<Box<dyn UnitOfWork>, RepositoryError> {
        let tx = self.db.inner_ref().begin().await?;
        Ok(Box::new(SolutionUnitOfWork::new(tx)))
    }
}

pub struct SolutionUnitOfWork {
    tx: Transaction<'static, Postgres>,
}

impl SolutionUnitOfWork {
    fn new(tx: Transaction<'static, Postgres>) -> Self {
        Self { tx }
    }
    fn conn(&mut self) -> &mut sqlx::PgConnection {
        self.tx.as_mut()
    }
}

#[async_trait]
impl UnitOfWork for SolutionUnitOfWork {
    fn solutions(&mut self) -> &mut dyn SolutionRespositoryTx {
        self
    }
    fn tags(&mut self) -> &mut dyn TagRepositoryTx {
        self
    }

    async fn commit(mut self: Box<Self>) -> Result<(), RepositoryError> {
        self.tx.commit().await?;
        Ok(())
    }
    async fn rollback(mut self: Box<Self>) -> Result<(), RepositoryError> {
        self.tx.rollback().await?;
        Ok(())
    }
}

#[async_trait]
impl SolutionRespositoryTx for SolutionUnitOfWork {
    async fn create(&mut self, s: &Solution) -> Result<Uuid, RepositoryError> {
        sqlx::query!(
            r#"INSERT INTO solutions (id, problem_id, user_id, body_md, submit_url)
               VALUES ($1,$2,$3,$4,$5)"#,
            s.id,
            s.problem_id,
            s.user_id,
            s.body_md,
            s.submit_url
        )
        .execute(self.conn())
        .await?;
        Ok(s.id)
    }
    async fn replace_tags(
        &mut self,
        solution_id: Uuid,
        tag_ids: &[Uuid],
    ) -> Result<(), RepositoryError> {
        sqlx::query!(
            "DELETE FROM solution_tags WHERE solution_id = $1 AND NOT (tag_id = ANY($2))",
            solution_id,
            tag_ids
        )
        .execute(self.conn())
        .await?;

        sqlx::query!(
            r#"INSERT INTO solution_tags (solution_id, tag_id)
               SELECT $1, x FROM UNNEST($2::uuid[]) AS t(x)
               ON CONFLICT DO NOTHING"#,
            solution_id,
            tag_ids
        )
        .execute(self.conn())
        .await?;

        Ok(())
    }
}

#[async_trait]
impl TagRepositoryTx for SolutionUnitOfWork {
    async fn upsert(&mut self, names: &[String]) -> Result<Vec<Uuid>, RepositoryError> {
        if names.is_empty() {
            return Ok(vec![]);
        }
        let rows = sqlx::query!(
            r#"
            WITH input(tag) AS (
              SELECT DISTINCT trim(t) FROM UNNEST($1::text[]) AS t WHERE trim(t) != ''
            ),
            ins AS (
              INSERT INTO tags(name)
              SELECT tag FROM input
              ON CONFLICT (name) DO NOTHING
              RETURNING id
            )
            SELECT id FROM ins
            UNION ALL
            SELECT t.id
            FROM tags t
            JOIN input i ON t.name = i.tag
            "#,
            names
        )
        .fetch_all(self.conn())
        .await?;

        Ok(rows.into_iter().filter_map(|r| r.id).collect())
    }
}
