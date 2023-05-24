use std::{io::Write, os::unix::net::UnixStream};

use anyhow::Result;
use clap::{Parser, Subcommand};
use sway_thumbwheel_proxy::{Message, DEFAULT_SOCKET_PATH};

#[derive(Parser)]
struct Args {
    /// Path where the server socket lives
    #[arg(short, long, default_value_t = DEFAULT_SOCKET_PATH.to_string())]
    socket_path: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Go to the previous workspace
    Prev,
    /// Go to the next workspace
    Next,
    /// Go to the previous workspace on the current output
    #[command(name = "prev_on_output")]
    PrevOnOutput,
    /// Go to the next workspace on the current output
    #[command(name = "next_on_output")]
    NextOnOutput,
}

impl From<Commands> for Message {
    fn from(value: Commands) -> Self {
        match value {
            Commands::Prev => Message::Prev,
            Commands::Next => Message::Next,
            Commands::PrevOnOutput => Message::PrevOnOutput,
            Commands::NextOnOutput => Message::NextOnOutput,
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    let message: Message = args.command.into();

    let ser_msg = bincode::serialize(&message)?;
    let mut stream = UnixStream::connect(&args.socket_path)?;
    stream.write_all(&ser_msg)?;

    Ok(())
}
