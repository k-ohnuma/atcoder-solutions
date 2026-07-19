use axum::{Router, routing::get, routing::post};
use registry::Registry;

use crate::handler::{
    solution::get_solutions_by_user_name_handler,
    user::{create_user_handler, delete_me_handler, get_me_handler, revoke_tokens_handler},
};

pub fn build_user_routers() -> Router<Registry> {
    let routers = Router::new()
        .route("/", post(create_user_handler))
        .route("/me", get(get_me_handler).delete(delete_me_handler))
        .route("/me/revoke", post(revoke_tokens_handler))
        .route(
            "/{user_name}/solutions",
            get(get_solutions_by_user_name_handler),
        );
    Router::new().nest("/users", routers)
}
