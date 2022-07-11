mod player;
mod commands;
mod events;
mod defaults;
mod constants;

pub use commands::Command;
pub use constants::*;
pub use defaults::*;
pub use events::Event;
pub use events::PositionChange;
pub use events::StateChange;
pub use player::Player;
pub use player::PlayerState;

use thiserror::Error;

use pcap_files::{Pcap, PcapNG};

#[derive(Clone, Debug, Error)]
pub enum PlayerError {
    #[error("Failed to initialize the Player")]
    PlayerInitError,
    #[error("File type `{0}` is not supported for playback")]
    FileTypeNotSupported(String),
    #[error("The command channel failed")]
    CommandChannelError,
}

// TODO From impl
pub enum Recording {
    Pcap(Pcap),
    PcapNg(PcapNG),
}