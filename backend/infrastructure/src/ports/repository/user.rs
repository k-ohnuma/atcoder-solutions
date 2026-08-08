use async_trait::async_trait;
use derive_new::new;
use domain::{
    error::{repository::RepositoryError, user::CreateUserRepositoryError},
    model::user::{User, id::UserId, name::UserName},
    ports::repository::user::UserRepository,
};
use sqlx::Row;

use crate::error::map_sqlx_error;
use crate::{database::ConnectionPool, model::user::UserRow};

#[derive(new)]
pub struct UserRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn create_user(&self, user: User) -> Result<(), CreateUserRepositoryError> {
        let inserted = sqlx::query_scalar!(
            r#"
            INSERT INTO users (id, role, user_name)
            VALUES ($1, $2, $3)
            ON CONFLICT (user_name) DO NOTHING
            RETURNING id
            "#,
            user.id.as_str(),
            user.role.to_string(),
            user.user_name.as_str(),
        )
        .fetch_optional(self.db.inner_ref())
        .await
        .map_err(map_sqlx_error)
        .map_err(CreateUserRepositoryError::from)?;

        if inserted.is_none() {
            return Err(CreateUserRepositoryError::UserNameAlreadyExists);
        }
        Ok(())
    }
    async fn find_by_uid(&self, uid: &UserId) -> Result<Option<User>, RepositoryError> {
        let user_row = sqlx::query_as!(
            UserRow,
            r#"
            SELECT id, role, user_name, created_at, updated_at
            FROM users
            WHERE id = $1
            "#,
            uid.as_str(),
        )
        .fetch_optional(self.db.inner_ref())
        .await
        .map_err(map_sqlx_error)?;

        user_row.map(TryInto::try_into).transpose()
    }

    async fn user_name_exists(&self, user_name: &UserName) -> Result<bool, RepositoryError> {
        let rec = sqlx::query!(
            r#"
                SELECT EXISTS(
                    SELECT 1
                    FROM users
                    WHERE user_name = $1
                ) AS "exists!"
            "#,
            user_name.as_str()
        )
        .fetch_one(self.db.inner_ref())
        .await
        .map_err(map_sqlx_error)?;

        Ok(rec.exists)
    }

    async fn delete_by_uid(&self, uid: &UserId) -> Result<bool, RepositoryError> {
        let result = sqlx::query!(
            r#"
            DELETE FROM users
            WHERE id = $1
            "#,
            uid.as_str()
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(map_sqlx_error)?;

        Ok(result.rows_affected() > 0)
    }

    async fn revoke_tokens_by_uid(&self, uid: &UserId) -> Result<bool, RepositoryError> {
        let result = sqlx::query(
            r#"
            UPDATE users
            SET token_revoked_before = NOW()
            WHERE id = $1
            "#,
        )
        .bind(uid.as_str())
        .execute(self.db.inner_ref())
        .await
        .map_err(map_sqlx_error)?;

        Ok(result.rows_affected() > 0)
    }

    async fn is_token_revoked(
        &self,
        uid: &UserId,
        issued_at: i64,
    ) -> Result<bool, RepositoryError> {
        let revoked = sqlx::query(
            r#"
            SELECT COALESCE(
                (EXTRACT(EPOCH FROM token_revoked_before)::BIGINT > $2),
                FALSE
            ) AS revoked
            FROM users
            WHERE id = $1
            "#,
        )
        .bind(uid.as_str())
        .bind(issued_at)
        .fetch_optional(self.db.inner_ref())
        .await
        .map_err(map_sqlx_error)?
        .map(|row| row.get::<bool, _>("revoked"));

        Ok(revoked.unwrap_or(true))
    }
}
