use std::sync::Arc;

use derive_new::new;
use domain::ports::repository::user::UserRepository;

use crate::model::user::{UserError, revoke_tokens::RevokeTokensOutput};

#[derive(new)]
pub struct RevokeTokensUsecase {
    user_repository: Arc<dyn UserRepository>,
}

impl RevokeTokensUsecase {
    pub async fn run(&self, uid: String) -> Result<RevokeTokensOutput, UserError> {
        self.user_repository
            .revoke_tokens_by_uid(&uid)
            .await
            .map_err(UserError::from)?;
        Ok(RevokeTokensOutput::new(uid))
    }
}
