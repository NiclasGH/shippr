use std::{fmt, path::PathBuf, string::FromUtf8Error};

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error)]
pub enum Error {
    #[error("The given directory is not a valid directory path")]
    InvalidDirectory,

    #[error("Either --namespace or --all-namespaces must be present")]
    NoNamespacePassed,

    #[error("Both the local and repository location are set. Only one can be set")]
    DuplicateLocation,

    #[error("{0} is not installed")]
    MissingTool(String),

    #[error("{0} is installed but is not working properly")]
    FaultyTool(String),

    #[error("The values-default.yaml file does not exist: path: {0}")]
    ValuesDefaultMissing(PathBuf),

    #[error("A profile was passed but no fitting values file exists: path: {0}")]
    ValuesProfileMissing(PathBuf),

    #[error("The release name could not be determined in that namespace")]
    CouldNotFigureOutReleaseName,

    #[error(
        "The given release name cannot be the path. As such it cannot contain characters like /"
    )]
    ReleaseNameIsPath,

    // external errors
    #[error("Configuration Error: {0}")]
    ConfigError(#[from] config::ConfigError),

    #[error("I/O Error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Failed to convert CLI stdout to string: {0}")]
    Utf8Error(#[from] FromUtf8Error),
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}
