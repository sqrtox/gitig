use crate::error::{Error, Result};
use platform_dirs::AppDirs;
use std::path::PathBuf;

const APP_NAME: &str = env!("CARGO_PKG_NAME");

pub fn get_profiles_dir() -> Result<PathBuf> {
    AppDirs::new(Some(APP_NAME), true)
        .ok_or(Error::AppDirsNotDetected)
        .map(|app_dirs| app_dirs.data_dir.join("profiles"))
}
