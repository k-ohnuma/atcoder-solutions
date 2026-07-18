use axum::{
    Router,
    routing::{get, patch, post},
};
use registry::Registry;

use crate::handler::solution::{
    create_comment_handler, create_solution_handler, delete_comment_handler,
    delete_solution_handler, get_comments_by_solution_id_handler, get_latest_solutions_handler,
    get_my_vote_status_handler, get_solution_by_solution_id_handler,
    get_solution_votes_count_handler, get_solutions_by_problems_id_handler,
    get_solutions_by_user_name_handler, unvote_solution_handler, update_comment_handler,
    update_solution_handler, vote_solution_handler,
};

pub fn build_solution_routers() -> Router<Registry> {
    let solution_routers = Router::new()
        .route("/", post(create_solution_handler))
        .route("/", get(get_latest_solutions_handler))
        .route(
            "/{solution_id}",
            get(get_solution_by_solution_id_handler)
                .patch(update_solution_handler)
                .delete(delete_solution_handler),
        )
        .route(
            "/{solution_id}/comments",
            get(get_comments_by_solution_id_handler).post(create_comment_handler),
        )
        .route("/problems", get(get_solutions_by_problems_id_handler))
        .route("/users", get(get_solutions_by_user_name_handler))
        .route(
            "/{solution_id}/votes",
            get(get_solution_votes_count_handler),
        )
        .route(
            "/{solution_id}/votes/me",
            get(get_my_vote_status_handler)
                .put(vote_solution_handler)
                .delete(unvote_solution_handler),
        );

    let comment_routers = Router::new().route(
        "/{comment_id}",
        patch(update_comment_handler).delete(delete_comment_handler),
    );

    Router::new()
        .nest("/solutions", solution_routers)
        .nest("/comments", comment_routers)
}
