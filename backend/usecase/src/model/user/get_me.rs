use derive_new::new;

#[derive(new, Debug)]
pub struct GetMeOutput {
    pub id: String,
    pub user_name: String,
}
