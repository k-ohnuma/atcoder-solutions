use axum::{
    Router,
    routing::{get, post},
};
use registry::Registry;

use crate::handler::solution::{create_solution_handler, get_solution_by_solution_id_handler, get_solutions_by_problems_id_handler};

pub fn build_solution_routers() -> Router<Registry> {
    let routers = Router::new()
        .route("/", post(create_solution_handler))
        .route("/", get(get_solution_by_solution_id_handler))
        .route("/problems", get(get_solutions_by_problems_id_handler));
    Router::new().nest("/solutions", routers)
}
