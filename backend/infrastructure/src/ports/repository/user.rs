use async_trait::async_trait;
use derive_new::new;
use domain::{
    error::repository::RepositoryError, model::user::User, ports::repository::user::UserRepository,
};

use crate::error::map_sqlx_error;
use crate::{database::ConnectionPool, model::user::UserRow};

#[derive(new)]
pub struct UserRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn create_user(&self, user: User) -> Result<(), RepositoryError> {
        let inserted = sqlx::query_scalar!(
            r#"
            INSERT INTO users (id, role, user_name)
            VALUES ($1, $2, $3)
            ON CONFLICT (user_name) DO NOTHING
            RETURNING id
            "#,
            user.id,
            user.role.to_string(),
            user.user_name,
        )
        .fetch_optional(self.db.inner_ref())
        .await
        .map_err(map_sqlx_error)?;

        if inserted.is_none() {
            return Err(RepositoryError::UniqueViolation(format!(
                "user_name: {}",
                user.user_name
            )));
        }
        Ok(())
    }
    async fn find_by_uid(&self, uid: &str) -> Result<User, RepositoryError> {
        let user_row = sqlx::query_as!(
            UserRow,
            r#"
            SELECT * FROM users WHERE id = $1
            "#,
            uid,
        )
        .fetch_one(self.db.inner_ref())
        .await
        .map_err(map_sqlx_error)?;

        Ok(user_row.try_into()?)
    }
}
