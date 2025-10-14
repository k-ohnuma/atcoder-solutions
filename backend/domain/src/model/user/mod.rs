use strum::{Display, EnumString};

#[derive(Debug, Display, EnumString, Default)]
#[strum(serialize_all = "lowercase")]
pub enum Role {
    #[default]
    User,
    Admin
}

pub struct User {
    pub id: String,
    pub role: Role,
    pub user_name: String,
    pub color: String,
}
