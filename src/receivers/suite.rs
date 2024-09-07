//! Configuration for the log inputs.

use std::{
    fs::{self, File},
    io,
    path::{Path, PathBuf},
};

use serde::Serialize;

pub enum LogProtocol {
    NDJSON,
}

/// A suite of open receivers for log messages.
///
/// This is set up *before* forking when the process is run in the background.
pub struct ReceiverSuite {
    pub(super) stdin_protocol: Option<LogProtocol>,
    pub(super) fifo: Option<ReceiverPath>,
}

/// An open receiver with a configured protocol.
pub(super) struct ReceiverPath {
    pub(super) protocol: LogProtocol,
    pub(super) path: PathBuf,
}

impl ReceiverSuite {
    /// Create a new receiver suite.
    pub fn new() -> ReceiverSuite {
        ReceiverSuite {
            stdin_protocol: None,
            fifo: None,
        }
    }

    /// Listen for NDJSON messages on stdin.
    pub fn listen_stdin_ndjson(&mut self) {
        self.stdin_protocol = Some(LogProtocol::NDJSON);
    }

    /// Listen for NDJSON messages on an open FIFO.
    pub fn listen_fifo_ndjson(&mut self, path: &Path) {
        assert!(self.fifo.is_none());
        self.fifo = Some(ReceiverPath {
            protocol: LogProtocol::NDJSON,
            path: path.to_path_buf(),
        });
    }

    pub fn connection_info(&self) -> ConnectionInfo {
        ConnectionInfo {
            log_fifo: self.fifo.as_ref().map(|fifo| fifo.path.clone()),
        }
    }

    /// Clean up the receiver suite, deleting opened files.
    pub fn cleanup(mut self) -> Result<(), io::Error> {
        self.do_cleanup()
    }

    /// Disable deletion (for parent process).
    pub fn drop_without_deleting(mut self) {
        self.fifo = None;
    }

    fn do_cleanup(&mut self) -> Result<(), io::Error> {
        if let Some(fifo) = self.fifo.take() {
            fs::remove_file(&fifo.path)?;
        }
        Ok(())
    }
}

impl Drop for ReceiverSuite {
    fn drop(&mut self) {
        self.do_cleanup();
    }
}

/// Connection information to share with clients.
#[derive(Debug, Serialize, Clone)]
pub struct ConnectionInfo {
    pub log_fifo: Option<PathBuf>,
}
