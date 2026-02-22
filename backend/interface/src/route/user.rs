use axum::{Router, routing::get, routing::post};
use registry::Registry;

use crate::handler::user::{create_user_handler, delete_me_handler, get_me_handler};

pub fn build_user_routers() -> Router<Registry> {
    let routers = Router::new()
        .route("/", post(create_user_handler))
        .route("/me", get(get_me_handler).delete(delete_me_handler));
    Router::new().nest("/users", routers)
}
