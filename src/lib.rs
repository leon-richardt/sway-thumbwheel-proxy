use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Prev,
    Next,
    PrevOnOutput,
    NextOnOutput,
}

impl AsRef<str> for Message {
    fn as_ref(&self) -> &str {
        match self {
            Message::Prev => "workspace prev",
            Message::Next => "workspace next",
            Message::PrevOnOutput => "workspace prev_on_output",
            Message::NextOnOutput => "workspace next_on_output",
        }
    }
}

pub static DEFAULT_SOCKET_PATH: &str = "/tmp/sway-thumbwheel-proxy";
