//! Core log message schema

use std::borrow::Cow;

use chrono::serde::ts_microseconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::level::PrimLogLevel;

/// Core schema for log messages.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct LogMessage<'a> {
    /// Log level (see [super::LogLevel] for defined level numbers).
    pub level: PrimLogLevel,
    /// Timestamp at which the log message was created.
    ///
    /// This is serialized as an integer number of microseconds.
    #[serde(with = "ts_microseconds")]
    pub timestamp: DateTime<Utc>,
    /// Logger name (dot-separated).
    #[serde(default)]
    pub name: Option<Cow<'a, str>>,
    /// A context or operation identifier.
    #[serde(default)]
    pub context_id: Option<Cow<'a, str>>,
    /// The host/process originating the log message.
    #[serde(default)]
    pub origin: Option<OriginRef<'a>>,
    /// The log message.
    pub message: Cow<'a, str>,
}

/// Origin or reference to previously-defined origin.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum OriginRef<'a> {
    Ref(Uuid),
    Full(LogOrigin<'a>),
}

/// Origin (host and process) for a log message.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct LogOrigin<'a> {
    /// Identifier for the log origin.
    pub origin_id: Option<Uuid>,
    /// Hostname originating the log message.
    #[serde(default)]
    pub hostname: Option<Cow<'a, str>>,
    /// Name of the process initiating the log message.
    #[serde(default)]
    pub process_name: Option<Cow<'a, str>>,
    /// ID of the process initiating the log message.
    #[serde(default)]
    pub process_id: Option<u32>,
    /// Name of the thread initiating the message.
    #[serde(default)]
    pub thread_name: Option<Cow<'a, str>>,
    /// ID of the thread initiating the message.
    #[serde(default)]
    pub thread_id: Option<u32>,
}
