//! Log schema definitions.

pub mod level;
pub mod message;

pub use level::LogLevel;
pub use message::{LogMessage, LogOrigin, OriginRef};
