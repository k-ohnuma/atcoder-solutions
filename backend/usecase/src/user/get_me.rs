use std::sync::Arc;

use derive_new::new;
use domain::model::user::id::UserId;
use domain::ports::repository::user::UserRepository;

use crate::{error::UserError, model::user::get_me::GetMeOutput};

#[derive(new)]
pub struct GetMeUsecase {
    user_repository: Arc<dyn UserRepository>,
}

impl GetMeUsecase {
    pub async fn run(&self, uid: UserId) -> Result<GetMeOutput, UserError> {
        let user = self
            .user_repository
            .find_by_uid(&uid)
            .await
            .map_err(UserError::from)?
            .ok_or_else(|| UserError::NotFound("user not found".to_string()))?;
        Ok(GetMeOutput::new(user.id, user.user_name))
    }
}
