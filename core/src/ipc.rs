use std::io::{BufRead, BufReader, Write};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub const DEFAULT_SOCKET_PATH: &str = "/tmp/salsa.sock";

#[derive(Debug, Serialize, Deserialize)]
pub enum Request {
    Ping,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    Pong,
    Error { message: String },
}

pub fn write_message<W: Write, T: Serialize>(mut writer: W, message: &T) -> anyhow::Result<()> {
    let payload = serde_json::to_vec(message)?;
    writer.write_all(&payload)?;
    writer.write_all(b"\n")?;
    Ok(())
}

pub fn read_message<R: std::io::Read, T: DeserializeOwned>(reader: R) -> anyhow::Result<T> {
    let mut buf_reader = BufReader::new(reader);
    let mut line = String::new();
    buf_reader.read_line(&mut line)?;
    let trimmed = line.trim();
    let message = serde_json::from_str(trimmed)?;
    Ok(message)
}
