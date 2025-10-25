use axum::{Json, extract::State};
use registry::Registry;
use shared::response::ApiResponse;

use crate::model::version::VersionResponse;

pub async fn version(State(_registry): State<Registry>) -> Json<ApiResponse<VersionResponse>> {
    let version = env!("CARGO_PKG_VERSION");
    Json(ApiResponse::ok(version.into()))
}
