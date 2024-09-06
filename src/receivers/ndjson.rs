//! Receive newline-delimited JSON.

use serde_json::from_str;
use tokio::io::AsyncRead;
use tokio_stream::{Stream, StreamExt};
use tokio_util::codec::{FramedRead, LinesCodec, LinesCodecError};

use crate::schema::LogMessage;

use super::LogRecvResult;

pub fn parse_ndjson<'r, R: AsyncRead>(chan: R) -> impl Stream<Item = LogRecvResult> {
    let chan = FramedRead::new(chan, LinesCodec::new());
    chan.map(decode_json)
}

fn decode_json(line: Result<String, LinesCodecError>) -> LogRecvResult {
    let line = line?;
    let msg: LogMessage = from_str(&line)?;
    Ok(msg)
}
