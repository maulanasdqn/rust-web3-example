use std::collections::HashMap;
use std::sync::RwLock;

use async_trait::async_trait;
use uuid::Uuid;
use voting_errors::AppError;

use crate::domain::{Poll, PollRepository};

pub struct MemoryPollRepository {
    polls: RwLock<HashMap<Uuid, Poll>>,
}

impl MemoryPollRepository {
    pub fn new() -> Self {
        Self {
            polls: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for MemoryPollRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl PollRepository for MemoryPollRepository {
    async fn create(&self, poll: Poll) -> Result<Poll, AppError> {
        let mut polls = self
            .polls
            .write()
            .map_err(|_| AppError::InternalError("Lock poisoned".to_string()))?;

        polls.insert(poll.id, poll.clone());
        Ok(poll)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Poll>, AppError> {
        let polls = self
            .polls
            .read()
            .map_err(|_| AppError::InternalError("Lock poisoned".to_string()))?;

        Ok(polls.get(&id).cloned())
    }

    async fn list_all(&self) -> Result<Vec<Poll>, AppError> {
        let polls = self
            .polls
            .read()
            .map_err(|_| AppError::InternalError("Lock poisoned".to_string()))?;

        Ok(polls.values().cloned().collect())
    }
}
