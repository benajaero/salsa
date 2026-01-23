use std::env;
use std::os::unix::net::UnixStream;
use std::path::PathBuf;

use directories::ProjectDirs;
use salsa_core::ipc::{read_message, write_message, Request, Response, DEFAULT_SOCKET_PATH};
use salsa_store::Store;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.iter().any(|arg| arg == "--ping") {
        ping_agent(DEFAULT_SOCKET_PATH)?;
        return Ok(());
    }

    if args.iter().any(|arg| arg == "--list") {
        let store = Store::open(default_db_path()?)?;
        let snippets = store.list_snippets()?;
        for snippet in snippets {
            println!("{} -> {}", snippet.trigger, snippet.label);
        }
        return Ok(());
    }

    println!("salsa-app starting... use --ping or --list");
    Ok(())
}

fn ping_agent(socket_path: &str) -> anyhow::Result<()> {
    let stream = UnixStream::connect(socket_path)?;
    write_message(&stream, &Request::Ping)?;
    let response: Response = read_message(&stream)?;
    match response {
        Response::Pong => println!("agent pong"),
        Response::Error { message } => println!("agent error: {message}"),
    }
    Ok(())
}

fn default_db_path() -> anyhow::Result<PathBuf> {
    let proj_dirs = ProjectDirs::from("com", "salsa", "Salsa")
        .ok_or_else(|| anyhow::anyhow!("unable to resolve app support dir"))?;
    let dir = proj_dirs.data_dir();
    std::fs::create_dir_all(dir)?;
    Ok(dir.join("salsa.db"))
}
