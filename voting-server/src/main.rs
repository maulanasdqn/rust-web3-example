use std::sync::Arc;

use axum::Router;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use voting_polls::application::{CastVote, CreatePoll, GetPoll, GetResults};
use voting_polls::infrastructure::http::{poll_routes, PollsState};
use voting_polls::infrastructure::persistence::{MemoryPollRepository, SolanaVoteRepository};
use voting_solana::{SolanaClient, SolanaConfig};

mod config;

use config::ServerConfig;

#[tokio::main]
async fn main() {
    init_tracing();

    let config = ServerConfig::from_env();
    let app = create_app();

    let listener = TcpListener::bind(&config.address())
        .await
        .expect("Failed to bind address");

    info!("Server running on http://{}", config.address());

    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();
}

fn create_app() -> Router {
    let solana_config = SolanaConfig::from_env();
    let solana_client = Arc::new(
        SolanaClient::new(solana_config).expect("Failed to create Solana client"),
    );

    let poll_repo = Arc::new(MemoryPollRepository::new());
    let vote_repo = Arc::new(SolanaVoteRepository::new(solana_client));

    let create_poll = Arc::new(CreatePoll::new(poll_repo.clone()));
    let get_poll = Arc::new(GetPoll::new(poll_repo.clone()));
    let cast_vote = Arc::new(CastVote::new(poll_repo.clone(), vote_repo.clone()));
    let get_results = Arc::new(GetResults::new(poll_repo, vote_repo));

    let polls_state = PollsState {
        create_poll,
        get_poll,
        cast_vote,
        get_results,
    };

    Router::new().nest("/api/v1", poll_routes(polls_state))
}
