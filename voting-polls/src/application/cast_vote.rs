use std::sync::Arc;

use uuid::Uuid;
use voting_errors::AppError;

use crate::domain::{PollRepository, Vote, VoteRepository};

pub struct CastVoteInput {
    pub poll_id: Uuid,
    pub option_index: u8,
    pub voter_pubkey: String,
}

pub struct CastVote {
    poll_repo: Arc<dyn PollRepository>,
    vote_repo: Arc<dyn VoteRepository>,
}

impl CastVote {
    pub fn new(
        poll_repo: Arc<dyn PollRepository>,
        vote_repo: Arc<dyn VoteRepository>,
    ) -> Self {
        Self {
            poll_repo,
            vote_repo,
        }
    }

    pub async fn execute(&self, input: CastVoteInput) -> Result<Vote, AppError> {
        let poll = self
            .poll_repo
            .find_by_id(input.poll_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Poll not found".to_string()))?;

        if !poll.option_exists(input.option_index) {
            return Err(AppError::BadRequest("Invalid option index".to_string()));
        }

        let already_voted = self
            .vote_repo
            .has_voted(input.poll_id, &input.voter_pubkey)
            .await?;

        if already_voted {
            return Err(AppError::Conflict("You have already voted".to_string()));
        }

        let vote = Vote::new(
            input.poll_id,
            input.option_index,
            input.voter_pubkey,
            String::new(),
        );

        self.vote_repo.record_vote(vote).await
    }
}
