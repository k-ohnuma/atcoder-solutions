use std::sync::Arc;

use derive_new::new;
use domain::ports::repository::user::UserRepository;

use crate::model::user::{UserError, delete_me::DeleteMeOutput};

#[derive(new)]
pub struct DeleteMeUsecase {
    user_repository: Arc<dyn UserRepository>,
}

impl DeleteMeUsecase {
    pub async fn run(&self, uid: String) -> Result<DeleteMeOutput, UserError> {
        self.user_repository
            .delete_by_uid(&uid)
            .await
            .map_err(UserError::from)?;
        Ok(DeleteMeOutput::new(uid))
    }
}
