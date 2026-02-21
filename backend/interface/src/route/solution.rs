use axum::{
    Router,
    routing::{get, post},
};
use registry::Registry;

use crate::handler::solution::{
    create_comment_handler, create_solution_handler, get_comments_by_solution_id_handler,
    get_my_vote_status_handler, get_solution_by_solution_id_handler,
    get_solution_votes_count_handler, get_solutions_by_problems_id_handler,
    unvote_solution_handler, vote_solution_handler,
};

pub fn build_solution_routers() -> Router<Registry> {
    let routers = Router::new()
        .route("/", post(create_solution_handler))
        .route("/", get(get_solution_by_solution_id_handler))
        .route("/problems", get(get_solutions_by_problems_id_handler))
        .route(
            "/votes",
            post(vote_solution_handler)
                .delete(unvote_solution_handler)
                .get(get_solution_votes_count_handler),
        )
        .route(
            "/comments",
            post(create_comment_handler).get(get_comments_by_solution_id_handler),
        )
        .route("/votes/me", get(get_my_vote_status_handler));
    Router::new().nest("/solutions", routers)
}
