use std::str::FromStr;

use chrono::{DateTime, Utc};
use domain::model::user::{Role, User};

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
            color: value.color,
            user_name: value.user_name,
        }
    }
}
