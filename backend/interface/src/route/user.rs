use axum::{Router, routing::post};
use registry::Registry;

use crate::handler::user::create_user_handler;

pub fn build_user_routers() -> Router<Registry> {
    let routers = Router::new().route("/", post(create_user_handler));
    Router::new().nest("/users", routers)
}
