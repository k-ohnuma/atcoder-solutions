use derive_new::new;
use domain::model::user::id::UserId;

#[derive(new, Debug)]
pub struct GetMeOutput {
    pub id: UserId,
    pub user_name: String,
}
