use axum::{
    routing::{get, post},
    Router,
};

use super::handlers::{
    cast_vote_handler, create_poll_handler, get_poll_handler, get_results_handler,
    list_polls_handler, PollsState,
};

pub fn poll_routes(state: PollsState) -> Router {
    Router::new()
        .route("/polls", post(create_poll_handler))
        .route("/polls", get(list_polls_handler))
        .route("/polls/{id}", get(get_poll_handler))
        .route("/polls/{id}/vote", post(cast_vote_handler))
        .route("/polls/{id}/results", get(get_results_handler))
        .with_state(state)
}
