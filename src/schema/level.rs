//! Log level definitions.

use std::fmt;
use std::str::FromStr;

use parse_display::{Display, FromStr};
use serde::{Deserialize, Serialize};
use strum::{EnumIter, FromRepr, IntoEnumIterator};
use thiserror::Error;

#[derive(Error, Debug, Clone)]
#[error("invalid log level")]
pub struct LevelParseError;

/// Named log levels.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter, FromRepr, Display, FromStr,
)]
#[repr(u8)]
#[display(style = "lowercase")]
pub enum NamedLogLevel {
    Trace = 5,
    Debug5,
    Debug4,
    Debug3,
    Debug2,
    Debug = 10,
    Info = 20,
    Notice = 25,
    Warn = 30,
    Error = 40,
    Critical = 50,
    Fatal = 60,
}

/// Primitive log level representation.
pub type PrimLogLevel = u8;

/// Log levels
#[derive(Debug, Clone, Deserialize, Serialize, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
#[serde(transparent)]
pub struct LogLevel {
    level: PrimLogLevel,
}

impl LogLevel {
    /// Get the named log level.
    pub fn named(&self) -> Option<NamedLogLevel> {
        for l in NamedLogLevel::iter() {
            if l as u8 == self.level {
                return Some(l);
            }
        }
        None
    }

    /// Get the primitive log level.
    pub fn primitive(&self) -> PrimLogLevel {
        self.level
    }
}

impl Default for LogLevel {
    fn default() -> Self {
        LogLevel {
            level: NamedLogLevel::Info as u8,
        }
    }
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(named) = self.named() {
            write!(f, "{}", named)
        } else {
            write!(f, "{}", self.level)
        }
    }
}

impl FromStr for LogLevel {
    type Err = LevelParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(lvl) = PrimLogLevel::from_str(s) {
            Ok(lvl.into())
        } else if let Ok(lvl) = NamedLogLevel::from_str(s) {
            Ok(lvl.into())
        } else {
            Err(LevelParseError)
        }
    }
}

impl From<PrimLogLevel> for LogLevel {
    fn from(value: PrimLogLevel) -> Self {
        LogLevel { level: value }
    }
}

impl Into<PrimLogLevel> for LogLevel {
    fn into(self) -> PrimLogLevel {
        self.level
    }
}

impl From<NamedLogLevel> for LogLevel {
    fn from(value: NamedLogLevel) -> Self {
        LogLevel { level: value as u8 }
    }
}

#[test]
fn test_parse_level() {
    assert_eq!(
        LogLevel::from_str("info")
            .expect("invalid level")
            .primitive(),
        20
    );
    assert_eq!(
        LogLevel::from_str("fatal")
            .expect("invalid level")
            .primitive(),
        60
    );
    assert_eq!(
        LogLevel::from_str("trace")
            .expect("invalid level")
            .primitive(),
        5
    );
    assert_eq!(
        LogLevel::from_str("debug")
            .expect("invalid level")
            .primitive(),
        10
    );
}

#[test]
fn test_level_named() {
    let mut level = LogLevel::from(40);
    assert_eq!(level.primitive(), 40);
    assert_eq!(level.named(), Some(NamedLogLevel::Error));

    level = 5.into();
    assert_eq!(level.named(), Some(NamedLogLevel::Trace));
}
