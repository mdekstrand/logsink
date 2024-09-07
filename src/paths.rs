//! Utilities for locating paths.
use std::{
    fs,
    path::{Path, PathBuf},
};

use tempdir::TempDir;
use xdg::BaseDirectories;

use crate::errors::SetupError;

/// Directory for working files / pipes / streams etc.
///
/// Implemented as a distinct type to deal with whether or not the directory
/// should be removed.
#[derive(Debug)]
pub enum WorkDir {
    Path(PathBuf),
    TempDir(TempDir),
}

impl AsRef<Path> for WorkDir {
    fn as_ref(&self) -> &Path {
        match self {
            WorkDir::Path(pth) => pth.as_ref(),
            WorkDir::TempDir(td) => td.as_ref(),
        }
    }
}

/// Get the runtime directory.
pub fn runtime_dir() -> Result<WorkDir, SetupError> {
    let xdg = BaseDirectories::with_prefix("logsink")?;
    if let Ok(path) = xdg.get_runtime_directory() {
        let mut path = path.clone();
        path.push("logsink");
        if !fs::exists(&path)? {
            fs::create_dir(&path)?;
        }
        Ok(WorkDir::Path(path))
    } else {
        Ok(WorkDir::TempDir(TempDir::new("logsink")?))
    }
}
