use chrono::{DateTime, NaiveDate, Utc};

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Created,
    InProgress,
    Done,
}

#[derive(Debug, Clone)]
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
            status: Status::Created,
            due,
            created_at: Utc::now(),
        }
    }
}
