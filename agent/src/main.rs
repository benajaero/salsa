use std::env;
use std::fs;
use std::os::unix::net::{UnixListener, UnixStream};

use salsa_core::ipc::{read_message, write_message, Request, Response, DEFAULT_SOCKET_PATH};

mod agent;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.iter().any(|arg| arg == "--serve") {
        return run_server(DEFAULT_SOCKET_PATH);
    }

    if args.iter().any(|arg| arg == "--run") {
        let agent = agent::Agent::new();
        agent.run();
        return Ok(());
    }

    println!("salsa-agent starting... use --serve or --run");
    Ok(())
}

fn run_server(socket_path: &str) -> anyhow::Result<()> {
    if std::path::Path::new(socket_path).exists() {
        fs::remove_file(socket_path)?;
    }

    let listener = UnixListener::bind(socket_path)?;
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_client(stream)?,
            Err(err) => eprintln!("IPC accept error: {err}"),
        }
    }
    Ok(())
}

fn handle_client(stream: UnixStream) -> anyhow::Result<()> {
    let request: Request = read_message(&stream)?;
    let response = match request {
        Request::Ping => Response::Pong,
    };
    write_message(&stream, &response)?;
    Ok(())
}
