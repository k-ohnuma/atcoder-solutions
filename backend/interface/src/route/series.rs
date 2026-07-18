use axum::{Router, routing::get};
use registry::Registry;

use crate::handler::{
    contest::get_contests_by_series_handler, problem::get_contest_group_by_contest_series_handler,
};

pub fn build_series_routers() -> Router<Registry> {
    let routers = Router::new()
        .route("/{series}/contests", get(get_contests_by_series_handler))
        .route(
            "/{series}/problem-groups",
            get(get_contest_group_by_contest_series_handler),
        );
    Router::new().nest("/series", routers)
}
