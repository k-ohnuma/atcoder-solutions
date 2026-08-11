use serde::Serialize;
use usecase::model::user::get_me::GetMeOutput;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMeResponse {
    pub id: String,
    pub user_name: String,
}

impl From<GetMeOutput> for GetMeResponse {
    fn from(value: GetMeOutput) -> Self {
        Self {
            id: value.id.into_string(),
            user_name: value.user_name.into_string(),
        }
    }
}
