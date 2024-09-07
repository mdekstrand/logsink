//! Receive newline-delimited JSON.

use serde_json::from_str;
use tokio::io::AsyncRead;
use tokio_stream::{Stream, StreamExt};
use tokio_util::codec::{FramedRead, LinesCodec, LinesCodecError};

use crate::schema::LogMessage;

use super::LogRecvResult;

pub fn parse_ndjson<C: AsyncRead + Send + Sync + Unpin>(
    input: C,
) -> impl Stream<Item = LogRecvResult> {
    let input = FramedRead::new(input, LinesCodec::new());
    return input.map(decode_json);
}

fn decode_json(line: Result<String, LinesCodecError>) -> LogRecvResult {
    let line = line?;
    let msg: LogMessage = from_str(&line)?;
    Ok(msg)
}
