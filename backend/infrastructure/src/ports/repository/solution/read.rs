use domain::{
    error::repository::RepositoryError,
    model::{
        problem::id::ProblemId,
        solution::id::SolutionId,
        user::{id::UserId, name::UserName},
    },
    read_model::solution::{
        SolutionDetails, SolutionListItem, SolutionListSort, UserSolutionListItem,
    },
};

use crate::{
    database::ConnectionPool,
    error::map_sqlx_error,
    model::solution::{
        SolutionDetailsViewRaw, SolutionListItemViewRaw, UserSolutionListItemViewRaw,
    },
};

pub(super) async fn get_latest_solutions(
    db: &ConnectionPool,
    limit: Option<i32>,
) -> Result<Vec<SolutionListItem>, RepositoryError> {
    let solutions = sqlx::query_as!(
        SolutionListItemViewRaw,
        r#"
            SELECT s.id, s.title, s.problem_id, s.user_id, u.user_name,
                   COUNT(sv.user_id) AS "votes_count!",
                   s.created_at, s.updated_at
            FROM solutions s
            JOIN users u on s.user_id = u.id
            LEFT JOIN solution_votes sv ON sv.solution_id = s.id
            GROUP BY s.id, s.title, s.problem_id, s.user_id, u.user_name, s.created_at, s.updated_at
            ORDER BY s.created_at DESC
            LIMIT COALESCE($1, 2147483647)
        "#,
        limit
    )
    .fetch_all(db.inner_ref())
    .await
    .map_err(map_sqlx_error)?;

    solutions.into_iter().map(TryInto::try_into).collect()
}

pub(super) async fn get_solutions_by_problem_id(
    db: &ConnectionPool,
    problem_id: ProblemId,
    sort: SolutionListSort,
    limit: Option<i32>,
) -> Result<Vec<SolutionListItem>, RepositoryError> {
    let problem_id_ref = problem_id.as_str();
    let solutions = match sort {
        SolutionListSort::Latest => {
            sqlx::query_as!(
                SolutionListItemViewRaw,
                r#"
                    SELECT s.id, s.title, s.problem_id, s.user_id, u.user_name,
                           COUNT(sv.user_id) AS "votes_count!",
                           s.created_at, s.updated_at
                    FROM solutions s
                    JOIN users u on s.user_id = u.id
                    LEFT JOIN solution_votes sv ON sv.solution_id = s.id
                    WHERE s.problem_id = $1
                    GROUP BY s.id, s.title, s.problem_id, s.user_id, u.user_name, s.created_at, s.updated_at
                    ORDER BY s.created_at DESC
                    LIMIT COALESCE($2, 2147483647)
                "#,
                problem_id_ref,
                limit
            )
            .fetch_all(db.inner_ref())
            .await
            .map_err(map_sqlx_error)?
        }
        SolutionListSort::Votes => {
            sqlx::query_as!(
                SolutionListItemViewRaw,
                r#"
                    SELECT s.id, s.title, s.problem_id, s.user_id, u.user_name,
                           COUNT(sv.user_id) AS "votes_count!",
                           s.created_at, s.updated_at
                    FROM solutions s
                    JOIN users u on s.user_id = u.id
                    LEFT JOIN solution_votes sv ON sv.solution_id = s.id
                    WHERE s.problem_id = $1
                    GROUP BY s.id, s.title, s.problem_id, s.user_id, u.user_name, s.created_at, s.updated_at
                    ORDER BY "votes_count!" DESC, s.created_at DESC
                    LIMIT COALESCE($2, 2147483647)
                "#,
                problem_id_ref,
                limit
            )
            .fetch_all(db.inner_ref())
            .await
            .map_err(map_sqlx_error)?
        }
    };

    solutions.into_iter().map(TryInto::try_into).collect()
}

pub(super) async fn get_solution_by_solution_id(
    db: &ConnectionPool,
    solution_id: SolutionId,
) -> Result<Option<SolutionDetails>, RepositoryError> {
    let solution = sqlx::query_as!(
        SolutionDetailsViewRaw,
        r#"
            SELECT
                s.id,
                s.title,
                s.problem_id,
                p.contest_code,
                p.title as problem_title,
                s.user_id,
                u.user_name,
                COALESCE(
                    array_remove(array_agg(t.name ORDER BY t.name), NULL),
                    ARRAY[]::text[]
                ) AS "tags!: Vec<String>",
                s.body_md,
                s.submit_url,
                s.created_at,
                s.updated_at
            FROM solutions s
            JOIN users u on s.user_id = u.id
            JOIN problems p on s.problem_id = p.id
            LEFT JOIN solution_tags st ON st.solution_id = s.id
            LEFT JOIN tags t ON t.id = st.tag_id
            WHERE s.id = $1
            GROUP BY
                s.id,
                s.title,
                s.problem_id,
                p.contest_code,
                p.title,
                s.user_id,
                u.user_name,
                s.body_md,
                s.submit_url,
                s.created_at,
                s.updated_at
        "#,
        solution_id.into_uuid()
    )
    .fetch_optional(db.inner_ref())
    .await
    .map_err(map_sqlx_error)?;

    solution.map(TryInto::try_into).transpose()
}

pub(super) async fn get_solutions_by_user_name(
    db: &ConnectionPool,
    user_name: UserName,
    sort: SolutionListSort,
) -> Result<Vec<UserSolutionListItem>, RepositoryError> {
    let user_name_ref = user_name.as_str();
    let rows = match sort {
        SolutionListSort::Latest => {
            sqlx::query_as!(
                UserSolutionListItemViewRaw,
                r#"
                    SELECT s.id, s.title, s.problem_id, p.title AS "problem_title!", s.user_id, u.user_name,
                           COUNT(sv.user_id) AS "votes_count!",
                           s.created_at, s.updated_at
                    FROM solutions s
                    JOIN users u ON s.user_id = u.id
                    JOIN problems p ON s.problem_id = p.id
                    LEFT JOIN solution_votes sv ON sv.solution_id = s.id
                    WHERE u.user_name = $1
                    GROUP BY s.id, s.title, s.problem_id, p.title, s.user_id, u.user_name, s.created_at, s.updated_at
                    ORDER BY s.created_at DESC
                "#,
                user_name_ref
            )
            .fetch_all(db.inner_ref())
            .await
            .map_err(map_sqlx_error)?
        }
        SolutionListSort::Votes => {
            sqlx::query_as!(
                UserSolutionListItemViewRaw,
                r#"
                    SELECT s.id, s.title, s.problem_id, p.title AS "problem_title!", s.user_id, u.user_name,
                           COUNT(sv.user_id) AS "votes_count!",
                           s.created_at, s.updated_at
                    FROM solutions s
                    JOIN users u ON s.user_id = u.id
                    JOIN problems p ON s.problem_id = p.id
                    LEFT JOIN solution_votes sv ON sv.solution_id = s.id
                    WHERE u.user_name = $1
                    GROUP BY s.id, s.title, s.problem_id, p.title, s.user_id, u.user_name, s.created_at, s.updated_at
                    ORDER BY "votes_count!" DESC, s.created_at DESC
                "#,
                user_name_ref
            )
            .fetch_all(db.inner_ref())
            .await
            .map_err(map_sqlx_error)?
        }
    };

    rows.into_iter().map(TryInto::try_into).collect()
}

pub(super) async fn solution_exists(
    db: &ConnectionPool,
    solution_id: SolutionId,
) -> Result<bool, RepositoryError> {
    let rec = sqlx::query!(
        r#"
            SELECT EXISTS(
                SELECT 1
                FROM solutions
                WHERE id = $1
            ) AS "exists!"
        "#,
        solution_id.into_uuid()
    )
    .fetch_one(db.inner_ref())
    .await
    .map_err(map_sqlx_error)?;

    Ok(rec.exists)
}

pub(super) async fn get_solution_user_id(
    db: &ConnectionPool,
    solution_id: SolutionId,
) -> Result<Option<UserId>, RepositoryError> {
    let rec = sqlx::query!(
        r#"
            SELECT user_id
            FROM solutions
            WHERE id = $1
        "#,
        solution_id.into_uuid()
    )
    .fetch_optional(db.inner_ref())
    .await
    .map_err(map_sqlx_error)?;

    rec.map(|row| UserId::try_new(row.user_id))
        .transpose()
        .map_err(|error| RepositoryError::Unexpected(error.to_string()))
}
