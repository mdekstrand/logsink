//! Utilities for locating paths.
use std::{
    env::temp_dir,
    fs::{self, remove_dir_all},
    io,
    ops::Deref,
    path::{Path, PathBuf},
};

use nix::unistd::mkdtemp;
use xdg::BaseDirectories;

use crate::errors::SetupError;

/// Directory for working files / pipes / streams etc.
///
/// Implemented as a distinct type to deal with whether or not the directory
/// should be removed.
#[derive(Debug)]
pub struct WorkDir {
    path: PathBuf,
    delete: bool,
}

impl WorkDir {
    /// Drop this handle *without* deleting the directory (to work with forking, etc.).
    pub fn drop_ref(mut self) {
        // just disable delete and let drop do its work
        self.delete = false;
    }

    pub fn cleanup(mut self) -> Result<(), io::Error> {
        self.do_cleanup()?;
        self.delete = false; // to prevent cleanup in drop
        Ok(())
    }

    fn do_cleanup(&self) -> Result<(), io::Error> {
        if self.delete {
            remove_dir_all(&self.path)?;
        }
        Ok(())
    }
}

impl Drop for WorkDir {
    #[allow(unused_must_use)]
    fn drop(&mut self) {
        self.do_cleanup();
    }
}

impl AsRef<Path> for WorkDir {
    fn as_ref(&self) -> &Path {
        self.path.as_path()
    }
}

impl Deref for WorkDir {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
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
        Ok(WorkDir {
            path,
            delete: false,
        })
    } else {
        let mut tmpdir = temp_dir();
        tmpdir.push("logsink-XXXXXXXX");
        let path = mkdtemp(&tmpdir)?;
        Ok(WorkDir { path, delete: true })
    }
}
