use std::sync::Arc;

use derive_new::new;
use domain::ports::repository::user::UserRepository;

use crate::model::user::{UserError, get_me::GetMeOutput};

#[derive(new)]
pub struct GetMeUsecase {
    user_repository: Arc<dyn UserRepository>,
}

impl GetMeUsecase {
    pub async fn run(&self, uid: String) -> Result<GetMeOutput, UserError> {
        let user = self
            .user_repository
            .find_by_uid(&uid)
            .await
            .map_err(UserError::from)?;
        Ok(GetMeOutput::new(user.id, user.user_name))
    }
}
