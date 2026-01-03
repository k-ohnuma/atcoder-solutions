use strum::{Display, EnumString};

#[derive(Debug, Display, EnumString, Default, Clone, Copy, PartialEq, Eq)]
#[strum(serialize_all = "lowercase")]
pub enum Role {
    #[default]
    User,
    Admin,
}

#[derive(Debug, Display, EnumString, Default, Clone, Copy, PartialEq, Eq)]
#[strum(serialize_all = "lowercase")]
pub enum Color {
    #[default]
    Gray,
    Brown,
    Green,
    Cyan,
    Blue,
    Yellow,
    Orange,
    Red,
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
    pub role: Role,
    pub user_name: String,
    pub color: Color,
}
