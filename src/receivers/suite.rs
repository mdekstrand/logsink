//! Configuration for the log inputs.

use std::{
    fs::{self, File},
    io,
    path::{Path, PathBuf},
};

pub enum LogProtocol {
    NDJSON,
}

/// A suite of open receivers for log messages.
///
/// This is set up *before* forking when the process is run in the background.
pub struct ReceiverSuite {
    pub(super) stdin_protocol: Option<LogProtocol>,
    pub(super) fifo: Option<OpenReceiver<File>>,
}

/// An open receiver with a configured protocol.
pub(super) struct OpenReceiver<R: 'static> {
    pub(super) channel: R,
    pub(super) protocol: LogProtocol,
    pub(super) path: Option<PathBuf>,
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
    pub fn listen_fifo_ndjson(&mut self, path: &Path, file: File) {
        assert!(self.fifo.is_none());
        self.fifo = Some(OpenReceiver {
            channel: file,
            protocol: LogProtocol::NDJSON,
            path: Some(path.to_path_buf()),
        });
    }

    /// Clean up the receiver suite, deleting opened files.
    pub fn cleanup(mut self) -> Result<(), io::Error> {
        self.do_cleanup()
    }

    /// Disable deletion (for parent process).
    pub fn drop_without_deleting(mut self) {
        if let Some(fifo) = self.fifo.as_mut() {
            fifo.path = None
        }
    }

    fn do_cleanup(&mut self) -> Result<(), io::Error> {
        if let Some(fifo) = self.fifo.as_mut() {
            if let Some(path) = fifo.path.take() {
                fs::remove_file(&path)?;
            }
        }
        Ok(())
    }
}

impl Drop for ReceiverSuite {
    fn drop(&mut self) {
        self.do_cleanup();
    }
}
