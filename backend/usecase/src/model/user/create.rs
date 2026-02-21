use derive_new::new;
use domain::model::user::{Role, User};
use validator::{Validate, ValidationError};

#[derive(Validate)]
pub struct CreateUserInput {
    #[validate(length(min = 1, max = 120), custom(function = "validate_not_blank"))]
    pub uid: String,
    #[validate(length(min = 1, max = 120), custom(function = "validate_not_blank"))]
    pub user_name: String,
}

fn validate_not_blank(value: &str) -> Result<(), ValidationError> {
    if value.trim().is_empty() {
        return Err(ValidationError::new("blank"));
    }
    Ok(())
}

impl From<CreateUserInput> for User {
    fn from(value: CreateUserInput) -> Self {
        Self {
            id: value.uid,
            role: Role::default(),
            user_name: value.user_name,
        }
    }
}

#[derive(new, Debug)]
pub struct CreateUserOutput {
    pub user_name: String,
}
