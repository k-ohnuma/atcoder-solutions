use derive_new::new;
use domain::model::user::{Role, User, id::UserId, name::UserName};
use validator::{Validate, ValidationError};

#[derive(Validate)]
pub struct CreateUserInput {
    pub user_name: UserName,
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
    pub fn try_new(
        user_name: String,
        email: String,
        password: String,
    ) -> Result<Self, crate::error::UserError> {
        Ok(Self {
            user_name: UserName::try_new(user_name)?,
            email,
            password,
        })
    }

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
    pub user_name: UserName,
}
