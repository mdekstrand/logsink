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
use receivers::{fifo::make_fifo, suite::ReceiverSuite};
use schema::level::LogLevel;
use tokio::{sync::broadcast, task::JoinSet};

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

// The main entry point.  Note that while we use Tokio, this is *not* decorated with
// `tokio::main` — if the client has requested that we fork into the background, we
// don't want to start up the Tokio runtime until *after* we fork.
fn main() -> Result<(), SetupError> {
    let cli = LogSinkCLI::parse();
    cli.run()
}

impl LogSinkCLI {
    fn run(&self) -> Result<(), SetupError> {
        let work = runtime_dir()?;
        let suite = self.prepare_suite()?;

        println!("{}", serde_json::to_string(&suite.connection_info())?);

        Ok(())
    }

    fn prepare_suite(&self) -> Result<ReceiverSuite, SetupError> {
        let work = runtime_dir()?;
        let mut suite = ReceiverSuite::new();

        if self.listen_fifo {
            let fifo_path = make_fifo(&work)?;
            suite.listen_fifo_ndjson(&fifo_path);
        }

        Ok(suite)
    }

    // async fn pump_messages(&self, suite: ReceiverSuite) {
    //     let (send, recv) = broadcast::channel(500);
    //     let listen = suite.pump_messages(send);
    // }
}
