//! Utilities for locating paths.
use std::path::PathBuf;

use xdg::BaseDirectories;

use crate::errors::SetupError;

/// Get the runtime directory.
pub fn runtime_dir() -> Result<PathBuf, SetupError> {
    let xdg = BaseDirectories::with_prefix("net.ekstrandom.logsink")?;
    if let Ok(path) = xdg.get_runtime_directory() {
        Ok(path.clone())
    } else {
        todo!()
    }
}
