use std::sync::Arc;

use voting_errors::AppError;

use crate::domain::{Poll, PollRepository};

pub struct CreatePollInput {
    pub title: String,
    pub options: Vec<String>,
}

pub struct CreatePoll {
    poll_repo: Arc<dyn PollRepository>,
}

impl CreatePoll {
    pub fn new(poll_repo: Arc<dyn PollRepository>) -> Self {
        Self { poll_repo }
    }

    pub async fn execute(&self, input: CreatePollInput) -> Result<Poll, AppError> {
        if input.title.is_empty() {
            return Err(AppError::ValidationError("Title cannot be empty".to_string()));
        }

        if input.options.len() < 2 {
            return Err(AppError::ValidationError(
                "Poll must have at least 2 options".to_string(),
            ));
        }

        let poll = Poll::new(input.title, input.options);
        self.poll_repo.create(poll).await
    }
}
