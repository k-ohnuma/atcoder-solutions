use std::str::FromStr;

use domain::model::user::Color;
use serde::{Deserialize, Serialize};
use usecase::user::{
    UserError,
    create_user::{CreateUserInput, CreateUserOutput},
};

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserRequest {
    pub user_name: String,
    pub color: String,
}

pub fn try_from_create_user_request_for_create_user_input(
    req: CreateUserRequest,
    uid: String,
) -> Result<CreateUserInput, UserError> {
    let color = Color::from_str(req.color.to_lowercase().as_str())
        .map_err(|_| UserError::BadRequest(format!("color: {}", req.color)))?;
    Ok(CreateUserInput {
        uid,
        user_name: req.user_name,
        color,
    })
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserResponse {
    user_name: String,
    color: String,
}

impl From<CreateUserOutput> for CreateUserResponse {
    fn from(value: CreateUserOutput) -> Self {
        Self {
            user_name: value.user_name,
            color: value.color.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use domain::model::user::Color;

    use super::{CreateUserRequest, try_from_create_user_request_for_create_user_input};

    #[test]
    fn test_try_from_create_user_request_for_create_user_input() {
        let req = CreateUserRequest {
            user_name: "user_name".into(),
            color: "gray".into(),
        };
        let con = try_from_create_user_request_for_create_user_input(req.to_owned(), "uid".into());
        assert!(con.is_ok());
        let colors = [
            ("red", Color::Red),
            ("orange", Color::Orange),
            ("yellow", Color::Yellow),
            ("blue", Color::Blue),
            ("Cyan", Color::Cyan),
            ("GREEN", Color::Green),
            ("broWn", Color::Brown),
            ("gray", Color::Gray),
        ];
        for &p in colors.iter() {
            let req = CreateUserRequest {
                user_name: "user".into(),
                color: p.0.to_owned(),
            };

            let inp =
                try_from_create_user_request_for_create_user_input(req.to_owned(), "uid".into())
                    .unwrap();
            assert_eq!(inp.color, p.1);
        }
    }
}
