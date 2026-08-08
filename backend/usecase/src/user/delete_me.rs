use std::sync::Arc;

use derive_new::new;
use domain::model::user::id::UserId;
use domain::ports::{external::auth::IdentityProviderPort, repository::user::UserRepository};
use tracing::error;

use crate::{error::UserError, model::user::delete_me::DeleteMeOutput};

const RECENT_LOGIN_MAX_AGE_SECONDS: i64 = 5 * 60;

#[derive(new)]
pub struct DeleteMeUsecase {
    user_repository: Arc<dyn UserRepository>,
    identity_provider: Arc<dyn IdentityProviderPort>,
}

impl DeleteMeUsecase {
    pub async fn run(
        &self,
        uid: UserId,
        auth_time: i64,
        now_epoch_seconds: i64,
    ) -> Result<DeleteMeOutput, UserError> {
        if now_epoch_seconds - auth_time > RECENT_LOGIN_MAX_AGE_SECONDS {
            return Err(UserError::RequiresRecentLogin(
                "recent login required".to_string(),
            ));
        }

        self.identity_provider
            .delete_user(&uid)
            .await
            .map_err(UserError::from)?;

        let deleted = self
            .user_repository
            .delete_by_uid(&uid)
            .await
            .map_err(|repository_error| {
                error!(
                    uid = uid.as_str(),
                    repository_error = ?repository_error,
                    "Firebase Auth user was deleted, but database user deletion failed; administrative cleanup may be required"
                );
                UserError::from(repository_error)
            })?;
        if !deleted {
            return Err(UserError::NotFound("user not found".to_string()));
        }
        Ok(DeleteMeOutput::new(uid))
    }
}
