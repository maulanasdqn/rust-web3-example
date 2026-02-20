use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;
use voting_errors::AppError;
use voting_types::{ListResponse, SingleResponse};

use crate::application::{CastVote, CreatePoll, GetPoll, GetResults};
use crate::infrastructure::http::dto::{
    CastVoteRequest, CreatePollRequest, PollResponse, VoteResponse, VoteResultsResponse,
};

#[derive(Clone)]
pub struct PollsState {
    pub create_poll: Arc<CreatePoll>,
    pub get_poll: Arc<GetPoll>,
    pub cast_vote: Arc<CastVote>,
    pub get_results: Arc<GetResults>,
}

pub async fn create_poll_handler(
    State(state): State<PollsState>,
    Json(payload): Json<CreatePollRequest>,
) -> Result<Json<SingleResponse<PollResponse>>, AppError> {
    let poll = state.create_poll.execute(payload.into()).await?;
    Ok(Json(SingleResponse::new(poll.into(), "Poll created")))
}

pub async fn list_polls_handler(
    State(state): State<PollsState>,
) -> Result<Json<ListResponse<PollResponse>>, AppError> {
    let polls = state.get_poll.list_all().await?;
    let responses: Vec<PollResponse> = polls.into_iter().map(Into::into).collect();
    Ok(Json(ListResponse::new(responses)))
}

pub async fn get_poll_handler(
    State(state): State<PollsState>,
    Path(id): Path<Uuid>,
) -> Result<Json<SingleResponse<PollResponse>>, AppError> {
    let poll = state.get_poll.execute(id).await?;
    Ok(Json(SingleResponse::new(poll.into(), "Poll retrieved")))
}

pub async fn cast_vote_handler(
    State(state): State<PollsState>,
    Path(poll_id): Path<Uuid>,
    Json(payload): Json<CastVoteRequest>,
) -> Result<Json<SingleResponse<VoteResponse>>, AppError> {
    let vote = state.cast_vote.execute(payload.into_input(poll_id)).await?;
    Ok(Json(SingleResponse::new(vote.into(), "Vote recorded")))
}

pub async fn get_results_handler(
    State(state): State<PollsState>,
    Path(poll_id): Path<Uuid>,
) -> Result<Json<SingleResponse<VoteResultsResponse>>, AppError> {
    let results = state.get_results.execute(poll_id).await?;
    Ok(Json(SingleResponse::new(results.into(), "Results retrieved")))
}
