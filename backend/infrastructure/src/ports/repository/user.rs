use async_trait::async_trait;
use derive_new::new;
use domain::{model::user::User, ports::repository::user::UserRepository};
use shared::error::repository::RepositoryError;

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
            INSERT INTO users (id, role, user_name, color)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (user_name) DO NOTHING
            RETURNING id
            "#,
            user.id,
            user.role.to_string(),
            user.user_name,
            user.color.to_string()
        )
        .fetch_optional(self.db.inner_ref())
        .await
        .map_err(RepositoryError::from)?;

        if inserted.is_none() {
            return Err(RepositoryError::UniqueViolation(format!(
                "user_name: {}",
                user.user_name
            )));
        }
        Ok(())
    }
    async fn find_by_uid(&self, uid: &str) -> Result<User, RepositoryError> {
        let user = sqlx::query_as!(
            UserRow,
            r#"
            SELECT * FROM users WHERE id = $1
            "#,
            uid,
        )
        .fetch_one(self.db.inner_ref())
        .await
        .map_err(RepositoryError::from)?;

        Ok(User::from(user))
    }
}
