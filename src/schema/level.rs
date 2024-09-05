//! Log level definitiions.

use parse_display::{Display, FromStr};
use strum::{EnumIter, FromRepr};

/// Named log levels for the unified logging layer.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, EnumIter, FromRepr, Display, FromStr)]
#[repr(u8)]
#[display(style = "lowercase")]
pub enum LogLevel {
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
