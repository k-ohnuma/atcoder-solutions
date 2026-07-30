use serde::{Deserialize, Serialize};
use usecase::{
    error::UserError,
    model::user::create::{CreateUserInput, CreateUserOutput},
};

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserRequest {
    pub user_name: String,
    pub email: String,
    pub password: String,
}

pub fn try_from_create_user_request_for_create_user_input(
    req: CreateUserRequest,
) -> Result<CreateUserInput, UserError> {
    Ok(CreateUserInput {
        user_name: req.user_name,
        email: req.email,
        password: req.password,
    })
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserResponse {
    user_name: String,
}

impl From<CreateUserOutput> for CreateUserResponse {
    fn from(value: CreateUserOutput) -> Self {
        Self {
            user_name: value.user_name,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{CreateUserRequest, try_from_create_user_request_for_create_user_input};

    #[test]
    fn test_try_from_create_user_request_for_create_user_input() {
        let req = CreateUserRequest {
            user_name: "user_name".into(),
            email: "user@example.com".into(),
            password: "Password1".into(),
        };
        let con = try_from_create_user_request_for_create_user_input(req.to_owned());
        assert!(con.is_ok());
    }
}
