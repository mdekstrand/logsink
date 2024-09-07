mod console;
mod formats;
mod receivers;
mod schema;

use clap::Parser;
use schema::level::LogLevel;

/// Collect and save log events.
#[derive(Parser, Debug)]
#[command(name = "logsink")]
struct LogSinkCLI {
    #[arg(short = 'L', long = "console-level", default_value = "info")]
    console_level: LogLevel,

    #[arg(long = "file-level", default_value = "info")]
    file_level: LogLevel,
}

fn main() {
    let cli = LogSinkCLI::parse();

    eprintln!("console level: {}", cli.console_level.primitive());
    eprintln!("file level: {}", cli.file_level.primitive());
}
