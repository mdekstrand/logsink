mod console;
mod errors;
mod formats;
mod paths;
mod receivers;
mod schema;

use std::path::PathBuf;

use clap::Parser;
use errors::SetupError;
use paths::runtime_dir;
use schema::level::LogLevel;

/// Collect and save log events.
#[derive(Parser, Debug)]
#[command(name = "logsink")]
struct LogSinkCLI {
    /// log level for console output
    #[arg(
        short = 'L',
        long = "console-level",
        default_value = "info",
        value_name = "LEVEL"
    )]
    console_level: LogLevel,

    /// log file
    #[arg(long = "log-file", value_name = "FILE")]
    log_file: Option<PathBuf>,
    /// log level for file output
    #[arg(long = "log-file-level", default_value = "info", value_name = "LEVEL")]
    file_level: LogLevel,

    /// create and open a FIFO to receive log events
    #[arg(long = "listen-fifo")]
    listen_fifo: bool,

    /// fork and put the log writer in the background
    #[arg(long = "background")]
    background: bool,
}

fn main() -> Result<(), SetupError> {
    let cli = LogSinkCLI::parse();

    eprintln!("console level: {}", cli.console_level.primitive());
    eprintln!("file level: {}", cli.file_level.primitive());
    eprintln!("runtime directory: {:?}", runtime_dir()?);
    Ok(())
}
