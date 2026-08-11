use derive_new::new;
use domain::model::user::{id::UserId, name::UserName};

#[derive(new, Debug)]
pub struct GetMeOutput {
    pub id: UserId,
    pub user_name: UserName,
}
