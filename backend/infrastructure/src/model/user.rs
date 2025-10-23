use std::str::FromStr;

use chrono::{DateTime, Utc};
use domain::model::user::{Color, Role, User};

pub struct UserRow {
    pub id: String,
    pub role: String,
    pub user_name: String,
    pub color: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<UserRow> for User {
    fn from(value: UserRow) -> Self {
        Self {
            id: value.id,
            role: Role::from_str(value.role.as_str()).expect("role must be admin or user"),
            color: Color::from_str(value.color.as_str())
                .unwrap_or_else(|_| panic!("color is invalid: {}", value.color.as_str())),
            user_name: value.user_name,
        }
    }
}
