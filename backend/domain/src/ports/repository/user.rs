use async_trait::async_trait;

use crate::error::repository::RepositoryError;
use crate::model::user::User;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(&self, user: User) -> Result<(), RepositoryError>;
    async fn find_by_uid(&self, uid: &str) -> Result<User, RepositoryError>;
}
