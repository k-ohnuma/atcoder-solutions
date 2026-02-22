use std::sync::Arc;

use derive_new::new;
use domain::{model::user::User, ports::repository::user::UserRepository};
use validator::Validate;

use crate::model::user::{
    UserError,
    create::{CreateUserInput, CreateUserOutput},
};

#[derive(new)]
pub struct CreateUserUsecase {
    user_repository: Arc<dyn UserRepository>,
}

impl CreateUserUsecase {
    pub async fn run(&self, input: CreateUserInput) -> Result<CreateUserOutput, UserError> {
        input
            .validate()
            .map_err(|e| UserError::BadRequest(e.to_string()))?;
        let user = User::from(input);
        self.user_repository
            .create_user(user.to_owned())
            .await
            .map_err(UserError::from)?;
        Ok(CreateUserOutput::new(user.user_name))
    }
}
