//! Log level definitions.

use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use smol_str::StrExt;
use strum::{EnumIter, FromRepr, IntoEnumIterator};
use thiserror::Error;

#[derive(Error, Debug, Clone)]
#[error("invalid log level")]
pub struct LevelParseError;

/// Named log levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter, FromRepr)]
#[repr(u8)]
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

    /// Get named level.
    pub fn approx_named(&self) -> NamedLogLevel {
        let mut level = NamedLogLevel::Trace;
        for name in NamedLogLevel::iter() {
            if name as u8 > self.level {
                break;
            } else {
                level = name;
            }
        }
        level
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

impl NamedLogLevel {
    /// Get a short (3-character) name for the log level.
    pub fn short(&self) -> &'static str {
        match self {
            NamedLogLevel::Trace => "TRC",
            NamedLogLevel::Debug5 => "DB5",
            NamedLogLevel::Debug4 => "DB4",
            NamedLogLevel::Debug3 => "DB3",
            NamedLogLevel::Debug2 => "DB2",
            NamedLogLevel::Debug => "DBG",
            NamedLogLevel::Info => "INF",
            NamedLogLevel::Notice => "NTC",
            NamedLogLevel::Warn => "WRN",
            NamedLogLevel::Error => "ERR",
            NamedLogLevel::Critical => "CRI",
            NamedLogLevel::Fatal => "FTL",
        }
    }

    /// Get a medium (4- or 5-character) name for the log level.
    pub fn medium(&self) -> &'static str {
        match self {
            NamedLogLevel::Trace => "TRACE",
            NamedLogLevel::Debug5 => "DBUG5",
            NamedLogLevel::Debug4 => "DBUG4",
            NamedLogLevel::Debug3 => "DBUG3",
            NamedLogLevel::Debug2 => "DBUG2",
            NamedLogLevel::Debug => "DEBUG",
            NamedLogLevel::Info => "INFO ",
            NamedLogLevel::Notice => "NOTIC",
            NamedLogLevel::Warn => "WARN ",
            NamedLogLevel::Error => "ERROR",
            NamedLogLevel::Critical => "CRIT ",
            NamedLogLevel::Fatal => "FATAL",
        }
    }

    /// Get a full name for the log level.
    pub fn full(&self) -> &'static str {
        match self {
            NamedLogLevel::Trace => "TRACE",
            NamedLogLevel::Debug5 => "DEBUG5",
            NamedLogLevel::Debug4 => "DEBUG4",
            NamedLogLevel::Debug3 => "DEBUG3",
            NamedLogLevel::Debug2 => "DEBUG2",
            NamedLogLevel::Debug => "DEBUG",
            NamedLogLevel::Info => "INFO",
            NamedLogLevel::Notice => "NOTICE",
            NamedLogLevel::Warn => "WARN",
            NamedLogLevel::Error => "ERROR",
            NamedLogLevel::Critical => "CRITICAL",
            NamedLogLevel::Fatal => "FATAL",
        }
    }
}

impl fmt::Display for NamedLogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let txt = match f.width() {
            Some(n) if n < 5 => self.short(),
            Some(n) if n < 7 => self.medium(),
            _ => self.full(),
        };
        if f.alternate() {
            f.write_str(&txt.to_uppercase_smolstr())
        } else {
            f.write_str(txt)
        }
    }
}

impl FromStr for NamedLogLevel {
    type Err = LevelParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "TRACE" | "TRC" => Ok(NamedLogLevel::Trace),
            "DEBUG5" | "DBUG5" | "DB5" => Ok(NamedLogLevel::Debug5),
            "DEBUG4" | "DBUG4" | "DB4" => Ok(NamedLogLevel::Debug4),
            "DEBUG3" | "DBUG3" | "DB3" => Ok(NamedLogLevel::Debug3),
            "DEBUG2" | "DBUG2" | "DB2" => Ok(NamedLogLevel::Debug2),
            "DEBUG" | "DBG" => Ok(NamedLogLevel::Debug),
            "INFO" | "INF" => Ok(NamedLogLevel::Info),
            "NOTICE" => Ok(NamedLogLevel::Notice),
            "WARN" | "WARNING" | "WRN" => Ok(NamedLogLevel::Warn),
            "ERROR" | "ERR" => Ok(NamedLogLevel::Error),
            "CRITICAL" | "CRIT" | "CRI" => Ok(NamedLogLevel::Critical),
            "FATAL" | "FTL" => Ok(NamedLogLevel::Fatal),

            _ => Err(LevelParseError),
        }
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

#[test]
fn test_approx_level() {
    let mut level = LogLevel::from(45);
    assert_eq!(level.primitive(), 45);
    assert_eq!(level.approx_named(), NamedLogLevel::Error);

    level = 2.into();
    assert_eq!(level.approx_named(), NamedLogLevel::Trace);

    level = 100.into();
    assert_eq!(level.approx_named(), NamedLogLevel::Fatal);
}
