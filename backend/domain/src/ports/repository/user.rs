use async_trait::async_trait;
use shared::error::repository::RepositoryError;

use crate::model::user::User;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(&self, user: User) -> Result<(), RepositoryError>;
}

