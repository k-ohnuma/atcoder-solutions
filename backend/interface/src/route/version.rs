use axum::{Router, routing::get};
use registry::Registry;

use crate::handler::version::version;

pub fn build_version_routers() -> Router<Registry> {
    let routers = Router::new().route("/", get(version));
    Router::new().nest("/version", routers)
}
