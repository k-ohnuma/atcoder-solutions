use async_trait::async_trait;
use derive_new::new;
use domain::{
    error::repository::RepositoryError, model::problem::ContestSeries,
    ports::repository::contest::ContestRepository, read_model::contests::ContestListItem,
};

use crate::error::map_sqlx_error;
use crate::{database::ConnectionPool, model::contests::ContestListItemViewRaw};

#[derive(new)]
pub struct ContestRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl ContestRepository for ContestRepositoryImpl {
    async fn get_contests_by_series(
        &self,
        series: ContestSeries,
    ) -> Result<Vec<ContestListItem>, RepositoryError> {
        let contests = sqlx::query_as!(
            ContestListItemViewRaw,
            r#"
                SELECT c.id, c.code, c.series_code, c.created_at, c.updated_at
                FROM contests c
                WHERE c.series_code = $1
                  AND EXISTS (
                    SELECT 1
                    FROM problems p
                    WHERE p.contest_code = c.code
                  )
                ORDER BY c.code DESC
            "#,
            series.to_string()
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(map_sqlx_error)?;
        Ok(contests.into_iter().map(ContestListItem::from).collect())
    }
}
