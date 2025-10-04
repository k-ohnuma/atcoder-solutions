use axum::{Router, routing::get};

use crate::handler::version::version;

pub fn build_version_routers() -> Router<()> {
    let routers = Router::new().route("/", get(version));
    Router::new().nest("/version", routers)
}
