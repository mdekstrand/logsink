//! Support for configuring FIFOs to receive data.

use std::fs::File;
use std::path::PathBuf;
use std::process;

use nix::sys::stat::Mode;
use nix::unistd::mkfifo;

use crate::errors::SetupError;
use crate::paths::WorkDir;

pub fn open_fifo(work: &WorkDir) -> Result<(PathBuf, File), SetupError> {
    let pid = process::id();
    let name = format!("log-channel-{}.fifo", pid);
    let mut path = work.to_path_buf();
    path.push(&name);
    mkfifo(&path, Mode::S_IRUSR | Mode::S_IWUSR)?;
    let file = File::open(&path)?;
    Ok((path, file))
}
