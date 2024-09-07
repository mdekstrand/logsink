//! Text output formats.

use crate::schema::LogMessage;

/// Basic text format for uncolored console output.
pub fn basic_text(msg: &LogMessage) -> String {
    let time = msg.timestamp.naive_local().time();
    let time = time.format("%H:%M:%S%.3f");
    let level = msg.level.approx_named();

    format!("[{time}] {level:5} {}", &msg.message)
}
