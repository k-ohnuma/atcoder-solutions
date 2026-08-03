use derive_new::new;
use domain::model::user::{Role, User, id::UserId};
use validator::{Validate, ValidationError};

#[derive(Validate)]
pub struct CreateUserInput {
    #[validate(
        length(min = 1, max = 120),
        custom(function = "validate_not_blank"),
        custom(function = "validate_user_name_format")
    )]
    pub user_name: String,
    #[validate(email)]
    pub email: String,
    #[validate(
        length(min = 1),
        custom(function = "validate_not_blank"),
        custom(function = "validate_password_format")
    )]
    pub password: String,
}

fn validate_not_blank(value: &str) -> Result<(), ValidationError> {
    if value.trim().is_empty() {
        return Err(ValidationError::new("blank"));
    }
    Ok(())
}

fn validate_user_name_format(value: &str) -> Result<(), ValidationError> {
    if value.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return Ok(());
    }
    Err(ValidationError::new("invalid_user_name_format"))
}

fn validate_password_format(value: &str) -> Result<(), ValidationError> {
    let has_lowercase = value.chars().any(|c| c.is_ascii_lowercase());
    let has_uppercase = value.chars().any(|c| c.is_ascii_uppercase());
    let has_digit = value.chars().any(|c| c.is_ascii_digit());
    let has_allowed_chars = value
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || "#$%&@^`~".contains(c));

    if value.len() >= 6 && has_lowercase && has_uppercase && has_digit && has_allowed_chars {
        return Ok(());
    }
    Err(ValidationError::new("invalid_password_format"))
}

impl CreateUserInput {
    pub fn into_user(self, uid: UserId) -> User {
        User {
            id: uid,
            role: Role::default(),
            user_name: self.user_name,
        }
    }
}

#[derive(new, Debug)]
pub struct CreateUserOutput {
    pub user_name: String,
}
