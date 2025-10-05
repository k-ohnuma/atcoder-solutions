use axum::{Router, routing::get};
use registry::Registry;

use crate::handler::health::health_check;

pub fn build_health_check_routers() -> Router<Registry> {
    let routers = Router::new().route("/", get(health_check));
    Router::new().nest("/health", routers)
}
