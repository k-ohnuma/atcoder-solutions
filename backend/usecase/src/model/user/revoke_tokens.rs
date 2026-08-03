use derive_new::new;
use domain::model::user::id::UserId;

#[derive(new, Debug)]
pub struct RevokeTokensOutput {
    pub id: UserId,
}
