use serde::Serialize;
use usecase::model::user::revoke_tokens::RevokeTokensOutput;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RevokeTokensResponse {
    pub id: String,
}

impl From<RevokeTokensOutput> for RevokeTokensResponse {
    fn from(value: RevokeTokensOutput) -> Self {
        Self { id: value.id }
    }
}
