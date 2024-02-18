use thiserror::Error;

pub type Result<T> = std::result::Result<T, MSheepError>;

#[derive(Error, Debug)]
pub enum MSheepError {
    #[error("Not implemented")]
    NotImplemented,
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Metadata lib error: {0}")]
    AudioTagError(#[from] audiotags::Error),
    #[error("Task error: {0}")]
    TaskError(String),
    #[error("Config error: {0}")]
    ConfigError(#[from] bakunin_config::errors::ConfigError),
}