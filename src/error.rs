pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("There are no profiles to choose from")]
    NoProfilesToChoose,
    #[error("Cannot find folder to save app data")]
    AppDirsNotDetected,
    #[error("Profile name \"{0}\" is not found")]
    ProfileNotFound(String),
    #[error("{0}")]
    GitConfig(String),
    #[error(transparent)]
    GitConfigParse(#[from] gix_config::parse::Error),
    #[error(transparent)]
    FromUtf8(#[from] std::string::FromUtf8Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    GixConfigFileInit(#[from] gix_config::file::init::Error),
    #[error(transparent)]
    Rusqlite(#[from] rusqlite::Error),
    #[error(transparent)]
    Dialoguer(#[from] dialoguer::Error),
}
