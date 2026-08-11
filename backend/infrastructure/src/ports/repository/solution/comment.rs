use domain::{
    error::repository::RepositoryError,
    model::{
        solution::{comment_body::CommentBody, comment_id::CommentId, id::SolutionId},
        user::id::UserId,
    },
    read_model::solution::SolutionComment,
};

use crate::{
    database::ConnectionPool, error::map_sqlx_error, model::solution::SolutionCommentViewRaw,
};

pub(super) async fn create_comment(
    db: &ConnectionPool,
    user_id: &UserId,
    solution_id: SolutionId,
    body: &CommentBody,
) -> Result<SolutionComment, RepositoryError> {
    let comment = sqlx::query_as!(
        SolutionCommentViewRaw,
        r#"
        INSERT INTO comments (user_id, solution_id, body_md)
        VALUES ($1, $2, $3)
        RETURNING
            id,
            user_id,
            (SELECT user_name FROM users WHERE id = $1) AS "user_name!",
            solution_id,
            body_md,
            created_at,
            updated_at
        "#,
        user_id.as_str(),
        solution_id.into_uuid(),
        body.as_str()
    )
    .fetch_one(db.inner_ref())
    .await
    .map_err(map_sqlx_error)?;

    comment.try_into()
}

pub(super) async fn update_comment(
    db: &ConnectionPool,
    comment_id: CommentId,
    body: &CommentBody,
) -> Result<SolutionComment, RepositoryError> {
    let comment = sqlx::query_as!(
        SolutionCommentViewRaw,
        r#"
        UPDATE comments c
        SET body_md = $2
        FROM users u
        WHERE c.id = $1
          AND u.id = c.user_id
        RETURNING
            c.id,
            c.user_id,
            u.user_name,
            c.solution_id,
            c.body_md,
            c.created_at,
            c.updated_at
        "#,
        comment_id.into_uuid(),
        body.as_str()
    )
    .fetch_one(db.inner_ref())
    .await
    .map_err(map_sqlx_error)?;

    comment.try_into()
}

pub(super) async fn delete_comment(
    db: &ConnectionPool,
    comment_id: CommentId,
) -> Result<(), RepositoryError> {
    sqlx::query!(
        r#"
        DELETE FROM comments
        WHERE id = $1
        "#,
        comment_id.into_uuid()
    )
    .execute(db.inner_ref())
    .await
    .map_err(map_sqlx_error)?;

    Ok(())
}

pub(super) async fn get_comments_by_solution_id(
    db: &ConnectionPool,
    solution_id: SolutionId,
) -> Result<Vec<SolutionComment>, RepositoryError> {
    let comments = sqlx::query_as!(
        SolutionCommentViewRaw,
        r#"
            SELECT c.id, c.user_id, u.user_name, c.solution_id, c.body_md, c.created_at, c.updated_at
            FROM comments c
            JOIN users u ON c.user_id = u.id
            WHERE c.solution_id = $1
            ORDER BY c.created_at ASC
        "#,
        solution_id.into_uuid()
    )
    .fetch_all(db.inner_ref())
    .await
    .map_err(map_sqlx_error)?;

    comments.into_iter().map(TryInto::try_into).collect()
}

pub(super) async fn comment_exists(
    db: &ConnectionPool,
    comment_id: CommentId,
) -> Result<bool, RepositoryError> {
    let rec = sqlx::query!(
        r#"
            SELECT EXISTS(
                SELECT 1
                FROM comments
                WHERE id = $1
            ) AS "exists!"
        "#,
        comment_id.into_uuid()
    )
    .fetch_one(db.inner_ref())
    .await
    .map_err(map_sqlx_error)?;

    Ok(rec.exists)
}

pub(super) async fn get_comment_user_id(
    db: &ConnectionPool,
    comment_id: CommentId,
) -> Result<Option<UserId>, RepositoryError> {
    let rec = sqlx::query!(
        r#"
            SELECT user_id
            FROM comments
            WHERE id = $1
        "#,
        comment_id.into_uuid()
    )
    .fetch_optional(db.inner_ref())
    .await
    .map_err(map_sqlx_error)?;

    rec.map(|row| UserId::try_new(row.user_id))
        .transpose()
        .map_err(|error| RepositoryError::Unexpected(error.to_string()))
}
