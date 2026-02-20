use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone)]
pub struct Poll {
    pub id: Uuid,
    pub title: String,
    pub options: Vec<PollOption>,
    pub created_at: DateTime<Utc>,
}

impl Poll {
    pub fn new(title: String, option_texts: Vec<String>) -> Self {
        let options = option_texts
            .into_iter()
            .enumerate()
            .map(|(index, text)| PollOption {
                index: index as u8,
                text,
            })
            .collect();

        Self {
            id: Uuid::new_v4(),
            title,
            options,
            created_at: Utc::now(),
        }
    }

    pub fn option_exists(&self, index: u8) -> bool {
        self.options.iter().any(|o| o.index == index)
    }
}

#[derive(Clone)]
pub struct PollOption {
    pub index: u8,
    pub text: String,
}
