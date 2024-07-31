pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid database name")]
    InvalidDatabaseName,
    #[error("Cannot find folder to save app data")]
    AppDirsNotDetected,
    #[error("Profile name is empty")]
    EmptyProfileName,
    #[error("Profile name \"{0}\" is not found")]
    ProfileNotFound(String),
    #[error("{0}")]
    GitConfig(String),
    #[error(transparent)]
    FromUtf8(#[from] std::string::FromUtf8Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    GixConfigFileInit(#[from] gix_config::file::init::Error),
}