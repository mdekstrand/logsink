//! Log schema definitions.

pub mod level;
pub mod message;

pub use level::NamedLogLevel;
pub use message::{LogMessage, LogOrigin, OriginRef};
