use domain::{error::repository::RepositoryError, model::solution::id::SolutionId};

use crate::{database::ConnectionPool, error::map_sqlx_error};

pub(super) async fn delete_solution(
    db: &ConnectionPool,
    solution_id: SolutionId,
) -> Result<(), RepositoryError> {
    sqlx::query!(
        r#"
        DELETE FROM solutions
        WHERE id = $1
        "#,
        solution_id.into_uuid()
    )
    .execute(db.inner_ref())
    .await
    .map_err(map_sqlx_error)?;

    Ok(())
}
