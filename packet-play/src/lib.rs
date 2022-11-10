mod player;
mod commands;
mod events;
pub mod defaults;
mod constants;

use std::ffi::OsStr;
use std::fs::File;
pub use commands::Command;
pub use constants::*;
pub use defaults::*;
pub use events::Event;
pub use events::PositionChange;
pub use events::StateChange;
pub use player::Player;
pub use player::PlayerState;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use clap::Parser;
use log::error;
use thiserror::Error;

use pcap_files::{Pcap, PcapNG};

#[derive(Parser, Debug)]
// #[clap(name = "packet-play")]
#[command(author, version, about, long_about = None)]
pub struct PlayerOptions {
    pub file: String,
    // #[arg(value_parser, try_from_str))]
    #[clap(short, long, default_value_t = SocketAddr::new(IpAddr::V4(Ipv4Addr::BROADCAST), defaults::DEFAULT_DEST_PORT))]
    pub destination: SocketAddr,
    #[clap(short = 's', long = "source", default_value_t = defaults::DEFAULT_SRC_PORT)]
    pub source_port: u16,
    #[clap(short, long, default_value_t = defaults::DEFAULT_TTL)]
    pub ttl: u32,
    #[clap(short, long)]
    pub auto_play_disable: bool,
}

impl PlayerOptions {
    pub fn new(file: String) -> Self {
        Self {
            file,
            destination: SocketAddr::new(IpAddr::V4(Ipv4Addr::BROADCAST), defaults::DEFAULT_DEST_PORT),
            source_port: DEFAULT_SRC_PORT,
            ttl: DEFAULT_TTL,
            auto_play_disable: false
        }
    }

    pub fn with_destination(mut self, destination: SocketAddr) -> Self {
        self.destination = destination;
        self
    }

    pub fn with_source_port(mut self, source_port: u16) -> Self {
        self.source_port = source_port;
        self
    }

    pub fn with_ttl(mut self, ttl: u32) -> Self {
        self.ttl = ttl;
        self
    }

    pub fn disable_auto_play(mut self) -> Self {
        self.auto_play_disable = true;
        self
    }
}

#[derive(Clone, Debug, Error)]
pub enum PlayerError {
    #[error("Failed to initialize the Player")]
    PlayerInitError,
    #[error("The command channel failed")]
    CommandChannelError,
}

#[derive(Clone, Debug, Error)]
pub enum FileError {
    #[error("File type `{0}` is not supported for playback")]
    FileTypeNotSupported(String),
    #[error("The provided path is not a file")]
    NotAFile,
    #[error("The provided path does not exist")]
    PathDoesNotExist,
    #[error("Failed to parse the file")]
    ParseError,
}

// TODO add other supported types here
const SUPPORTED_EXTENSIONS: [&str; 1] = ["pcap"];
#[derive(Debug)]
pub enum Recording {
    Pcap(Pcap),
    PcapNg(PcapNG),
}

impl TryFrom<&str> for Recording {
    type Error = FileError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let file_path = std::path::Path::new(value);
        if !file_path.exists() {
            return Err(FileError::PathDoesNotExist);
        };
        if !file_path.is_file() {
            return Err(FileError::NotAFile);
        }

        match file_path.extension().and_then(OsStr::to_str) {
            None => { return Err(FileError::FileTypeNotSupported(String::from(""))); }
            Some(os_str) => {
                if !SUPPORTED_EXTENSIONS.contains(&os_str) {
                    return Err(FileError::FileTypeNotSupported(os_str.to_string()));
                }
            }
        }

        let file = File::open(file_path).expect("Could not open file.");

        return if let Ok(recording) = Pcap::try_from(file) {
            Ok(Recording::Pcap(recording))
        } else {
            Err(FileError::ParseError)
        }
    }
}