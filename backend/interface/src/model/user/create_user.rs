use serde::{Deserialize, Serialize};
use usecase::model::user::{
    UserError,
    create::{CreateUserInput, CreateUserOutput},
};

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserRequest {
    pub user_name: String,
}

pub fn try_from_create_user_request_for_create_user_input(
    req: CreateUserRequest,
    uid: String,
) -> Result<CreateUserInput, UserError> {
    Ok(CreateUserInput {
        uid,
        user_name: req.user_name,
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
        };
        let con = try_from_create_user_request_for_create_user_input(req.to_owned(), "uid".into());
        assert!(con.is_ok());
    }
}
