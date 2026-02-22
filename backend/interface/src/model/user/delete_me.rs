use serde::Serialize;
use usecase::model::user::delete_me::DeleteMeOutput;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteMeResponse {
    pub id: String,
}

impl From<DeleteMeOutput> for DeleteMeResponse {
    fn from(value: DeleteMeOutput) -> Self {
        Self { id: value.id }
    }
}
