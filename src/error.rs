use thiserror::Error;

#[derive(Error, Debug)]
pub enum TaskError {
    #[error("Invalid date format (expected YYYY-MM-DD)")]
    InvalidDate,

    #[error("Could not determine data directory")]
    NoDataDir,

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Json error: {0}")]
    Json(#[from] serde_json::Error),
}
