use strum::{Display, EnumString};

pub mod id;

use id::UserId;

#[derive(Debug, Display, EnumString, Default, Clone, Copy, PartialEq, Eq)]
#[strum(serialize_all = "lowercase")]
pub enum Role {
    #[default]
    User,
    Admin,
}
#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub role: Role,
    pub user_name: String,
}
