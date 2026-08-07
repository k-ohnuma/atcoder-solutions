use crate::error::map_sqlx_error;
use crate::{
    database::ConnectionPool,
    model::solution::{
        SolutionCommentViewRaw, SolutionDetailsViewRaw, SolutionListItemViewRaw,
        UserSolutionListItemViewRaw,
    },
};
use async_trait::async_trait;
use derive_new::new;
use domain::{
    error::repository::RepositoryError,
    model::{
        problem::id::ProblemId,
        solution::{comment_body::CommentBody, comment_id::CommentId, id::SolutionId},
        user::{id::UserId, name::UserName},
    },
    ports::repository::solution::SolutionRepository,
    read_model::solution::{
        SolutionComment, SolutionDetails, SolutionListItem, SolutionListSort, UserSolutionListItem,
    },
};

#[derive(new)]
pub struct SolutionRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl SolutionRepository for SolutionRepositoryImpl {
    async fn delete_solution(&self, solution_id: SolutionId) -> Result<(), RepositoryError> {
        let solution_id = solution_id.into_uuid();
        sqlx::query!(
            r#"
            DELETE FROM solutions
            WHERE id = $1
            "#,
            solution_id
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(map_sqlx_error)?;

        Ok(())
    }

    async fn vote_solution(
        &self,
        user_id: &UserId,
        solution_id: SolutionId,
    ) -> Result<(), RepositoryError> {
        let solution_id = solution_id.into_uuid();
        sqlx::query!(
            r#"
            INSERT INTO solution_votes (user_id, solution_id)
            VALUES ($1, $2)
            ON CONFLICT (user_id, solution_id) DO NOTHING
            "#,
            user_id.as_str(),
            solution_id
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(map_sqlx_error)?;

        Ok(())
    }

    async fn unvote_solution(
        &self,
        user_id: &UserId,
        solution_id: SolutionId,
    ) -> Result<(), RepositoryError> {
        let solution_id = solution_id.into_uuid();
        sqlx::query!(
            r#"
            DELETE FROM solution_votes
            WHERE user_id = $1 AND solution_id = $2
            "#,
            user_id.as_str(),
            solution_id
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(map_sqlx_error)?;

        Ok(())
    }

    async fn create_comment(
        &self,
        user_id: &UserId,
        solution_id: SolutionId,
        body: &CommentBody,
    ) -> Result<SolutionComment, RepositoryError> {
        let solution_id = solution_id.into_uuid();
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
            solution_id,
            body.as_str()
        )
        .fetch_one(self.db.inner_ref())
        .await
        .map_err(map_sqlx_error)?;

        comment.try_into()
    }

    async fn update_comment(
        &self,
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
        .fetch_one(self.db.inner_ref())
        .await
        .map_err(map_sqlx_error)?;

        comment.try_into()
    }

    async fn delete_comment(&self, comment_id: CommentId) -> Result<(), RepositoryError> {
        sqlx::query!(
            r#"
            DELETE FROM comments
            WHERE id = $1
            "#,
            comment_id.into_uuid()
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(map_sqlx_error)?;

        Ok(())
    }

    async fn get_latest_solutions(
        &self,
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
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(map_sqlx_error)?;

        solutions.into_iter().map(TryInto::try_into).collect()
    }

    async fn get_solutions_by_problem_id(
        &self,
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
                .fetch_all(self.db.inner_ref())
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
                .fetch_all(self.db.inner_ref())
                .await
                .map_err(map_sqlx_error)?
            }
        };

        solutions.into_iter().map(TryInto::try_into).collect()
    }
    async fn get_solution_by_solution_id(
        &self,
        solution_id: SolutionId,
    ) -> Result<SolutionDetails, RepositoryError> {
        let solution_id = solution_id.into_uuid();
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
            solution_id
        )
        .fetch_one(self.db.inner_ref())
        .await
        .map_err(map_sqlx_error)?;

        solution.try_into()
    }

    async fn get_solutions_by_user_name(
        &self,
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
                .fetch_all(self.db.inner_ref())
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
                .fetch_all(self.db.inner_ref())
                .await
                .map_err(map_sqlx_error)?
            }
        };

        rows.into_iter().map(TryInto::try_into).collect()
    }

    async fn get_solution_votes_count(
        &self,
        solution_id: SolutionId,
    ) -> Result<i64, RepositoryError> {
        let solution_id = solution_id.into_uuid();
        let rec = sqlx::query!(
            r#"
                SELECT COUNT(*) AS "count!" FROM solution_votes
                WHERE solution_id = $1
            "#,
            solution_id
        )
        .fetch_one(self.db.inner_ref())
        .await
        .map_err(map_sqlx_error)?;

        Ok(rec.count)
    }

    async fn has_user_voted_solution(
        &self,
        user_id: &UserId,
        solution_id: SolutionId,
    ) -> Result<bool, RepositoryError> {
        let solution_id = solution_id.into_uuid();
        let rec = sqlx::query!(
            r#"
                SELECT EXISTS(
                    SELECT 1
                    FROM solution_votes
                    WHERE user_id = $1 AND solution_id = $2
                ) AS "exists!"
            "#,
            user_id.as_str(),
            solution_id
        )
        .fetch_one(self.db.inner_ref())
        .await
        .map_err(map_sqlx_error)?;

        Ok(rec.exists)
    }

    async fn solution_exists(&self, solution_id: SolutionId) -> Result<bool, RepositoryError> {
        let solution_id = solution_id.into_uuid();
        let rec = sqlx::query!(
            r#"
                SELECT EXISTS(
                    SELECT 1
                    FROM solutions
                    WHERE id = $1
                ) AS "exists!"
            "#,
            solution_id
        )
        .fetch_one(self.db.inner_ref())
        .await
        .map_err(map_sqlx_error)?;

        Ok(rec.exists)
    }

    async fn get_comments_by_solution_id(
        &self,
        solution_id: SolutionId,
    ) -> Result<Vec<SolutionComment>, RepositoryError> {
        let solution_id = solution_id.into_uuid();
        let comments = sqlx::query_as!(
            SolutionCommentViewRaw,
            r#"
                SELECT c.id, c.user_id, u.user_name, c.solution_id, c.body_md, c.created_at, c.updated_at
                FROM comments c
                JOIN users u ON c.user_id = u.id
                WHERE c.solution_id = $1
                ORDER BY c.created_at ASC
            "#,
            solution_id
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(map_sqlx_error)?;

        comments.into_iter().map(TryInto::try_into).collect()
    }

    async fn get_solution_user_id(
        &self,
        solution_id: SolutionId,
    ) -> Result<UserId, RepositoryError> {
        let solution_id = solution_id.into_uuid();
        let rec = sqlx::query!(
            r#"
                SELECT user_id
                FROM solutions
                WHERE id = $1
            "#,
            solution_id
        )
        .fetch_one(self.db.inner_ref())
        .await
        .map_err(map_sqlx_error)?;

        UserId::try_new(rec.user_id).map_err(|error| RepositoryError::Unexpected(error.to_string()))
    }

    async fn comment_exists(&self, comment_id: CommentId) -> Result<bool, RepositoryError> {
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
        .fetch_one(self.db.inner_ref())
        .await
        .map_err(map_sqlx_error)?;

        Ok(rec.exists)
    }

    async fn get_comment_user_id(&self, comment_id: CommentId) -> Result<UserId, RepositoryError> {
        let rec = sqlx::query!(
            r#"
                SELECT user_id
                FROM comments
                WHERE id = $1
            "#,
            comment_id.into_uuid()
        )
        .fetch_one(self.db.inner_ref())
        .await
        .map_err(map_sqlx_error)?;

        UserId::try_new(rec.user_id).map_err(|error| RepositoryError::Unexpected(error.to_string()))
    }
}
