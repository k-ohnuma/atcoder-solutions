use std::sync::Arc;

use derive_new::new;
use domain::model::user::id::UserId;
use domain::ports::repository::user::UserRepository;

use crate::{error::UserError, model::user::revoke_tokens::RevokeTokensOutput};

#[derive(new)]
pub struct RevokeTokensUsecase {
    user_repository: Arc<dyn UserRepository>,
}

impl RevokeTokensUsecase {
    pub async fn run(&self, uid: UserId) -> Result<RevokeTokensOutput, UserError> {
        let revoked = self
            .user_repository
            .revoke_tokens_by_uid(&uid)
            .await
            .map_err(UserError::from)?;
        if !revoked {
            return Err(UserError::NotFound("user not found".to_string()));
        }
        Ok(RevokeTokensOutput::new(uid))
    }
}
