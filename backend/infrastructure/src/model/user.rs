use std::str::FromStr;

use chrono::{DateTime, Utc};
use domain::{
    error::repository::RepositoryError,
    model::user::{Role, User},
};

pub struct UserRow {
    pub id: String,
    pub role: String,
    pub user_name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TryFrom<UserRow> for User {
    type Error = RepositoryError;
    fn try_from(value: UserRow) -> Result<User, Self::Error> {
        let role = Role::from_str(value.role.as_str())
            .map_err(|e| RepositoryError::Unexpected(e.to_string()))?;
        Ok(User {
            id: value.id,
            role,
            user_name: value.user_name,
        })
    }
}
