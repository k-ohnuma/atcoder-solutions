use axum::extract::State;
use registry::Registry;

use crate::model::version::VersionResponse;
use crate::response::ApiResponse;

pub async fn version(State(_registry): State<Registry>) -> ApiResponse<VersionResponse> {
    let version = env!("CARGO_PKG_VERSION");
    ApiResponse::ok(version.into())
}
