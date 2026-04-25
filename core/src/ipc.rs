//! Minimal IPC protocol between the Salsa agent and UI.
//!
//! Messages are newline-delimited JSON over a Unix domain socket.
//! This is intentionally simple for v1; XPC or Mach ports may replace it later.

use std::io::{BufRead, BufReader, Write};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

/// Default socket path. The agent removes any stale file before binding.
pub const DEFAULT_SOCKET_PATH: &str = "/tmp/salsa.sock";

/// Request sent from UI to agent.
#[derive(Debug, Serialize, Deserialize)]
pub enum Request {
    /// Health-check.
    Ping,
}

/// Response sent from agent to UI.
#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    Pong,
    Error { message: String },
}

/// Serialise a message and append a newline.
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
