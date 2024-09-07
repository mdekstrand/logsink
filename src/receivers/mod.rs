//! Receivers (streams and protocols) for log messages.

use thiserror::Error;
use tokio_util::codec::LinesCodecError;

use crate::schema::LogMessage;

pub mod fifo;
pub mod ndjson;
pub mod suite;

/// Error receiving log messages.
#[derive(Error, Debug)]
pub enum LogRecvError {
    #[error("receiver IO error: {0}")]
    IO(#[from] std::io::Error),
    #[error("decoding failed: {0}")]
    Decode(&'static str),
    #[error("JSON parsing failed: {0}")]
    JSON(#[from] serde_json::Error),
}

impl From<LinesCodecError> for LogRecvError {
    fn from(value: LinesCodecError) -> Self {
        match value {
            LinesCodecError::MaxLineLengthExceeded => LogRecvError::Decode("line too long"),
            LinesCodecError::Io(err) => LogRecvError::IO(err),
        }
    }
}

pub type LogRecvResult<T = LogMessage> = Result<T, LogRecvError>;
