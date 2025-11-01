use axum::{Router, routing::post};
use registry::Registry;

use crate::handler::solution::create_solution_handler;

pub fn build_solution_routers() -> Router<Registry> {
    let routers = Router::new().route("/", post(create_solution_handler));
    Router::new().nest("/solutions", routers)
}
