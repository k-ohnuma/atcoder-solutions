use axum::{Json, extract::State};
use registry::Registry;

use crate::model::version::VersionResponse;

pub async fn version(State(_registry): State<Registry>) -> Json<VersionResponse> {
    let version = env!("CARGO_PKG_VERSION");
    Json(version.into())
}
