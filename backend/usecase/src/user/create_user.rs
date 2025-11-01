use std::sync::Arc;

use derive_new::new;
use domain::{
    model::user::{Color, Role, User},
    ports::repository::user::UserRepository,
};

use super::UserError;

pub struct CreateUserInput {
    pub uid: String,
    pub user_name: String,
    pub color: Color,
}

impl From<CreateUserInput> for User {
    fn from(value: CreateUserInput) -> Self {
        Self {
            id: value.uid,
            role: Role::default(),
            color: value.color,
            user_name: value.user_name,
        }
    }
}

#[derive(new, Debug)]
pub struct CreateUserOutput {
    pub user_name: String,
    pub color: Color,
}

#[derive(new)]
pub struct CreateUserUsecase {
    user_repository: Arc<dyn UserRepository>,
}

impl CreateUserUsecase {
    pub async fn run(&self, input: CreateUserInput) -> Result<CreateUserOutput, UserError> {
        let user = User::from(input);
        self.user_repository
            .create_user(user.to_owned())
            .await
            .map_err(UserError::from)?;
        Ok(CreateUserOutput::new(user.user_name, user.color))
    }
}
