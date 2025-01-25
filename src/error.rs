use std::string::FromUtf8Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("The given directory is not a valid path")]
    InvalidDirectory,

    #[error("Configuration Error: {0}")]
    ConfigError(#[from] config::ConfigError),

    #[error("Both the local and repository location are set. Only one can be set")]
    DuplicateLocation,

    #[error("I/O Error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Failed to convert CLI stdout to string: {0}")]
    Utf8Error(#[from] FromUtf8Error),
}
