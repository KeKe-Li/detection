use thiserror::Error;

#[derive(Error, Debug)]
pub enum MonitorError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Config error: {0}")]
    Config(String),

    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("Logger error: {0}")]
    Logger(String),
} 