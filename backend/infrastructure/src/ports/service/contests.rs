use async_trait::async_trait;
use derive_new::new;
use domain::error::repository::RepositoryError;
use domain::model::problem::ContestSeries;
use usecase::{model::contests::ContestListItem, service::contest::ContestService};

use crate::error::map_sqlx_error;
use crate::{database::ConnectionPool, model::contests::ContestListItemViewRaw};

#[derive(new)]
pub struct ContestServiceImpl {
    db: ConnectionPool,
}

#[async_trait]
impl ContestService for ContestServiceImpl {
    async fn get_contents_by_series(
        &self,
        series: ContestSeries,
    ) -> Result<Vec<ContestListItem>, RepositoryError> {
        let contests = sqlx::query_as!(
            ContestListItemViewRaw,
            r#"
                SELECT *
                FROM contests
                WHERE contests.series_code = $1
                ORDER BY contests.code DESC
            "#,
            series.to_string()
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(map_sqlx_error)?;
        Ok(contests.into_iter().map(ContestListItem::from).collect())
    }
}
