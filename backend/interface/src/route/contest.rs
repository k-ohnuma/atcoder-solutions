use axum::{Router, routing::get};
use registry::Registry;

use crate::handler::problem::get_problems_by_contest_handler;

pub fn build_contests_routers() -> Router<Registry> {
    let routers = Router::new().route(
        "/{contest_code}/problems",
        get(get_problems_by_contest_handler),
    );
    Router::new().nest("/contests", routers)
}
