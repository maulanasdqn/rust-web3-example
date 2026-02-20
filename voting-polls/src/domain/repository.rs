use async_trait::async_trait;
use uuid::Uuid;
use voting_errors::AppError;

use super::{Poll, Vote};

#[async_trait]
pub trait PollRepository: Send + Sync {
    async fn create(&self, poll: Poll) -> Result<Poll, AppError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Poll>, AppError>;
    async fn list_all(&self) -> Result<Vec<Poll>, AppError>;
}

#[async_trait]
pub trait VoteRepository: Send + Sync {
    async fn record_vote(&self, vote: Vote) -> Result<Vote, AppError>;
    async fn get_votes_for_poll(&self, poll_id: Uuid) -> Result<Vec<Vote>, AppError>;
    async fn has_voted(&self, poll_id: Uuid, voter_pubkey: &str) -> Result<bool, AppError>;
}
