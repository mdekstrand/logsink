use std::io;

use thiserror::Error;

/// Error in setting up the logging system.
#[derive(Error, Debug)]
pub enum SetupError {
    #[error("XDG layout error: {0}")]
    XDGError(#[from] xdg::BaseDirectoriesError),
    #[error("setup IO error: {0}")]
    IO(#[from] io::Error),
    #[error("setup Unix error: {0}")]
    Unix(#[from] nix::Error),
}
