use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::application::{CreatePollInput, CastVoteInput, OptionResult, VoteResults};
use crate::domain::{Poll, PollOption, Vote};

#[derive(Deserialize)]
pub struct CreatePollRequest {
    pub title: String,
    pub options: Vec<String>,
}

impl From<CreatePollRequest> for CreatePollInput {
    fn from(req: CreatePollRequest) -> Self {
        Self {
            title: req.title,
            options: req.options,
        }
    }
}

#[derive(Deserialize)]
pub struct CastVoteRequest {
    pub option_index: u8,
    pub voter_pubkey: String,
}

impl CastVoteRequest {
    pub fn into_input(self, poll_id: Uuid) -> CastVoteInput {
        CastVoteInput {
            poll_id,
            option_index: self.option_index,
            voter_pubkey: self.voter_pubkey,
        }
    }
}

#[derive(Serialize)]
pub struct PollResponse {
    pub id: Uuid,
    pub title: String,
    pub options: Vec<PollOptionResponse>,
    pub created_at: String,
}

#[derive(Serialize)]
pub struct PollOptionResponse {
    pub index: u8,
    pub text: String,
}

impl From<Poll> for PollResponse {
    fn from(poll: Poll) -> Self {
        Self {
            id: poll.id,
            title: poll.title,
            options: poll.options.into_iter().map(Into::into).collect(),
            created_at: poll.created_at.to_rfc3339(),
        }
    }
}

impl From<PollOption> for PollOptionResponse {
    fn from(option: PollOption) -> Self {
        Self {
            index: option.index,
            text: option.text,
        }
    }
}

#[derive(Serialize)]
pub struct VoteResponse {
    pub poll_id: Uuid,
    pub option_index: u8,
    pub voter_pubkey: String,
    pub transaction_signature: String,
}

impl From<Vote> for VoteResponse {
    fn from(vote: Vote) -> Self {
        Self {
            poll_id: vote.poll_id,
            option_index: vote.option_index,
            voter_pubkey: vote.voter_pubkey,
            transaction_signature: vote.transaction_signature,
        }
    }
}

#[derive(Serialize)]
pub struct VoteResultsResponse {
    pub poll: PollResponse,
    pub results: Vec<OptionResultResponse>,
    pub total_votes: u64,
}

#[derive(Serialize)]
pub struct OptionResultResponse {
    pub index: u8,
    pub text: String,
    pub votes: u64,
    pub percentage: f64,
}

impl From<VoteResults> for VoteResultsResponse {
    fn from(results: VoteResults) -> Self {
        Self {
            poll: results.poll.into(),
            results: results.results.into_iter().map(Into::into).collect(),
            total_votes: results.total_votes,
        }
    }
}

impl From<OptionResult> for OptionResultResponse {
    fn from(result: OptionResult) -> Self {
        Self {
            index: result.index,
            text: result.text,
            votes: result.votes,
            percentage: result.percentage,
        }
    }
}
