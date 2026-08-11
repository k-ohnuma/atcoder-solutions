use domain::{
    error::repository::RepositoryError,
    model::{solution::id::SolutionId, user::id::UserId},
};

use crate::{database::ConnectionPool, error::map_sqlx_error};

pub(super) async fn vote_solution(
    db: &ConnectionPool,
    user_id: &UserId,
    solution_id: SolutionId,
) -> Result<(), RepositoryError> {
    sqlx::query!(
        r#"
        INSERT INTO solution_votes (user_id, solution_id)
        VALUES ($1, $2)
        ON CONFLICT (user_id, solution_id) DO NOTHING
        "#,
        user_id.as_str(),
        solution_id.into_uuid()
    )
    .execute(db.inner_ref())
    .await
    .map_err(map_sqlx_error)?;

    Ok(())
}

pub(super) async fn unvote_solution(
    db: &ConnectionPool,
    user_id: &UserId,
    solution_id: SolutionId,
) -> Result<(), RepositoryError> {
    sqlx::query!(
        r#"
        DELETE FROM solution_votes
        WHERE user_id = $1 AND solution_id = $2
        "#,
        user_id.as_str(),
        solution_id.into_uuid()
    )
    .execute(db.inner_ref())
    .await
    .map_err(map_sqlx_error)?;

    Ok(())
}

pub(super) async fn get_solution_votes_count(
    db: &ConnectionPool,
    solution_id: SolutionId,
) -> Result<i64, RepositoryError> {
    let rec = sqlx::query!(
        r#"
            SELECT COUNT(*) AS "count!" FROM solution_votes
            WHERE solution_id = $1
        "#,
        solution_id.into_uuid()
    )
    .fetch_one(db.inner_ref())
    .await
    .map_err(map_sqlx_error)?;

    Ok(rec.count)
}

pub(super) async fn has_user_voted_solution(
    db: &ConnectionPool,
    user_id: &UserId,
    solution_id: SolutionId,
) -> Result<bool, RepositoryError> {
    let rec = sqlx::query!(
        r#"
            SELECT EXISTS(
                SELECT 1
                FROM solution_votes
                WHERE user_id = $1 AND solution_id = $2
            ) AS "exists!"
        "#,
        user_id.as_str(),
        solution_id.into_uuid()
    )
    .fetch_one(db.inner_ref())
    .await
    .map_err(map_sqlx_error)?;

    Ok(rec.exists)
}
