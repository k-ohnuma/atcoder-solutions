use derive_new::new;

#[derive(new, Debug)]
pub struct RevokeTokensOutput {
    pub id: String,
}
