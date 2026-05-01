use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, clap::ValueEnum)]
pub enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Status {
    Pending,
    Done,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub priority: Priority,
    pub status: Status,
    pub due: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(id: u32, title: String, priority: Priority, due: Option<NaiveDate>) -> Self {
        Self {
            id,
            title,
            priority,
            status: Status::Pending,
            due,
            created_at: Utc::now(),
        }
    }
}
