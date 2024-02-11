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
}