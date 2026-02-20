use std::sync::Arc;

use uuid::Uuid;
use voting_errors::AppError;

use crate::domain::{Poll, PollRepository};

pub struct GetPoll {
    poll_repo: Arc<dyn PollRepository>,
}

impl GetPoll {
    pub fn new(poll_repo: Arc<dyn PollRepository>) -> Self {
        Self { poll_repo }
    }

    pub async fn execute(&self, id: Uuid) -> Result<Poll, AppError> {
        self.poll_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Poll with id {} not found", id)))
    }

    pub async fn list_all(&self) -> Result<Vec<Poll>, AppError> {
        self.poll_repo.list_all().await
    }
}
