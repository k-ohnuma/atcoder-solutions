use axum::{Router, routing::get};
use registry::Registry;

use crate::handler::problem::{
    get_contest_group_by_contest_series_handler, get_problems_by_contest_handler,
};

pub fn build_problem_routers() -> Router<Registry> {
    let routers = Router::new()
        .route("/", get(get_problems_by_contest_handler))
        .route(
            "/contest-group",
            get(get_contest_group_by_contest_series_handler),
        );
    Router::new().nest("/problems", routers)
}
