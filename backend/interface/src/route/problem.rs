use axum::{Router, routing::get};
use registry::Registry;

use crate::handler::{
    problem::get_problem_by_id_handler, solution::get_solutions_by_problems_id_handler,
};

pub fn build_problem_routers() -> Router<Registry> {
    let routers = Router::new()
        .route(
            "/{problem_id}/solutions",
            get(get_solutions_by_problems_id_handler),
        )
        .route("/{problem_id}", get(get_problem_by_id_handler));
    Router::new().nest("/problems", routers)
}
