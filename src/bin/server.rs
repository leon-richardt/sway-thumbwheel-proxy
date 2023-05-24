use std::fs;
use std::os::unix::net::{UnixListener, UnixStream};
use std::time::{Duration, Instant};

use anyhow::Result;
use clap::{arg, Parser};
use swayipc::Connection;

use sway_thumbwheel_proxy::{Message, DEFAULT_SOCKET_PATH};

#[derive(Parser)]
struct Args {
    /// Duration (in milliseconds) during which to debounce commands
    #[arg(short, long, default_value_t = 200)]
    debounce_millis: u64,

    /// Path where the server socket should live
    #[arg(short, long, default_value_t = DEFAULT_SOCKET_PATH.to_string())]
    socket_path: String,
}

struct ServerState {
    debounce_dur: Duration,
    last_msg: Instant,
    sway: Connection,
}

fn handle(state: &mut ServerState, stream: UnixStream) -> Result<()> {
    let now = Instant::now();

    let msg: Message = bincode::deserialize_from(stream)?;

    if state.last_msg + state.debounce_dur >= now {
        return Ok(());
    }

    state.last_msg = now;
    state.sway.run_command(msg)?;

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut state = ServerState {
        debounce_dur: Duration::from_millis(args.debounce_millis),
        last_msg: Instant::now(),
        sway: Connection::new()?,
    };

    let listener = UnixListener::bind(&args.socket_path)?;

    let handler_copy = args.socket_path;
    ctrlc::set_handler(move || {
        // Ignore errors on file removal
        let _ = fs::remove_file(&handler_copy);
        std::process::exit(0);
    })?;

    for stream in listener.incoming() {
        handle(&mut state, stream?)?;
    }

    Ok(())
}
