use derive_new::new;
use domain::model::user::{Color, Role, User};

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
