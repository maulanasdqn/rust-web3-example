use std::sync::Arc;

use uuid::Uuid;
use voting_errors::AppError;

use crate::domain::{Poll, PollRepository, Vote, VoteRepository};

pub struct VoteResults {
    pub poll: Poll,
    pub results: Vec<OptionResult>,
    pub total_votes: u64,
}

pub struct OptionResult {
    pub index: u8,
    pub text: String,
    pub votes: u64,
    pub percentage: f64,
}

pub struct GetResults {
    poll_repo: Arc<dyn PollRepository>,
    vote_repo: Arc<dyn VoteRepository>,
}

impl GetResults {
    pub fn new(
        poll_repo: Arc<dyn PollRepository>,
        vote_repo: Arc<dyn VoteRepository>,
    ) -> Self {
        Self {
            poll_repo,
            vote_repo,
        }
    }

    pub async fn execute(&self, poll_id: Uuid) -> Result<VoteResults, AppError> {
        let poll = self
            .poll_repo
            .find_by_id(poll_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Poll not found".to_string()))?;

        let votes = self.vote_repo.get_votes_for_poll(poll_id).await?;
        let total_votes = votes.len() as u64;

        let results = Self::calculate_results(&poll, &votes, total_votes);

        Ok(VoteResults {
            poll,
            results,
            total_votes,
        })
    }

    fn calculate_results(poll: &Poll, votes: &[Vote], total: u64) -> Vec<OptionResult> {
        poll.options
            .iter()
            .map(|option| {
                let count = votes.iter().filter(|v| v.option_index == option.index).count() as u64;
                let percentage = if total > 0 {
                    (count as f64 / total as f64) * 100.0
                } else {
                    0.0
                };

                OptionResult {
                    index: option.index,
                    text: option.text.clone(),
                    votes: count,
                    percentage,
                }
            })
            .collect()
    }
}
