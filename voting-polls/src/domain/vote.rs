use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone)]
pub struct Vote {
    pub poll_id: Uuid,
    pub option_index: u8,
    pub voter_pubkey: String,
    pub transaction_signature: String,
    pub created_at: DateTime<Utc>,
}

impl Vote {
    pub fn new(
        poll_id: Uuid,
        option_index: u8,
        voter_pubkey: String,
        transaction_signature: String,
    ) -> Self {
        Self {
            poll_id,
            option_index,
            voter_pubkey,
            transaction_signature,
            created_at: Utc::now(),
        }
    }
}
