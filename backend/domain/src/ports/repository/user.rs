use async_trait::async_trait;

use crate::error::repository::RepositoryError;
use crate::model::user::User;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(&self, user: User) -> Result<(), RepositoryError>;
    async fn find_by_uid(&self, uid: &str) -> Result<User, RepositoryError>;
    async fn delete_by_uid(&self, uid: &str) -> Result<(), RepositoryError>;
    async fn revoke_tokens_by_uid(&self, uid: &str) -> Result<(), RepositoryError>;
    async fn is_token_revoked(&self, uid: &str, issued_at: i64) -> Result<bool, RepositoryError>;
}
