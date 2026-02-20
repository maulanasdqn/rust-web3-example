use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use uuid::Uuid;
use voting_errors::AppError;
use voting_solana::SolanaClient;

use crate::domain::{Vote, VoteRepository};

pub struct SolanaVoteRepository {
    solana_client: Arc<SolanaClient>,
    votes: RwLock<HashMap<Uuid, Vec<Vote>>>,
}

impl SolanaVoteRepository {
    pub fn new(solana_client: Arc<SolanaClient>) -> Self {
        Self {
            solana_client,
            votes: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl VoteRepository for SolanaVoteRepository {
    async fn record_vote(&self, mut vote: Vote) -> Result<Vote, AppError> {
        let memo = serde_json::json!({
            "poll_id": vote.poll_id.to_string(),
            "option": vote.option_index,
            "voter": vote.voter_pubkey
        })
        .to_string();

        let signature = self.solana_client.send_memo(&memo)?;
        vote.transaction_signature = signature.to_string();

        let mut votes = self
            .votes
            .write()
            .map_err(|_| AppError::InternalError("Lock poisoned".to_string()))?;

        votes
            .entry(vote.poll_id)
            .or_default()
            .push(vote.clone());

        Ok(vote)
    }

    async fn get_votes_for_poll(&self, poll_id: Uuid) -> Result<Vec<Vote>, AppError> {
        let votes = self
            .votes
            .read()
            .map_err(|_| AppError::InternalError("Lock poisoned".to_string()))?;

        Ok(votes.get(&poll_id).cloned().unwrap_or_default())
    }

    async fn has_voted(&self, poll_id: Uuid, voter_pubkey: &str) -> Result<bool, AppError> {
        let votes = self
            .votes
            .read()
            .map_err(|_| AppError::InternalError("Lock poisoned".to_string()))?;

        let has_voted = votes
            .get(&poll_id)
            .map(|v| v.iter().any(|vote| vote.voter_pubkey == voter_pubkey))
            .unwrap_or(false);

        Ok(has_voted)
    }
}
