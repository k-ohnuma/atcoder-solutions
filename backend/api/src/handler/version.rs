use axum::Json;

use crate::model::version::VersionResponse;

pub async fn version() -> Json<VersionResponse> {
    let version = env!("CARGO_PKG_VERSION");
    Json(version.into())
}
