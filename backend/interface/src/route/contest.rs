use axum::{Router, routing::get};
use registry::Registry;

use crate::handler::contest::get_contests_by_series_handler;

pub fn build_contests_routers() -> Router<Registry> {
    let routers = Router::new().route("/", get(get_contests_by_series_handler));
    Router::new().nest("/contests", routers)
}
