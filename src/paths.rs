//! Utilities for locating paths.
use std::path::PathBuf;

use xdg::BaseDirectories;

use crate::errors::SetupError;

/// Get the runtime directory.
pub fn runtime_dir() -> Result<PathBuf, SetupError> {
    let xdg = BaseDirectories::with_prefix("logsink")?;
    if let Ok(path) = xdg.get_runtime_directory() {
        let mut path = path.clone();
        path.push("logsink");
        Ok(path)
    } else {
        todo!()
    }
}
