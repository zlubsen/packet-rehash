mod tui;
pub mod input;
mod actions;

use std::env;
use std::process::exit;
use std::sync::mpsc;

use clap::Parser;
use log::error;
use packet_play::{Player, PlayerOptions, Recording};

const ERROR_CANNOT_START : i32 = 1;
const ERROR_RUNTIME : i32 = 2;

fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    tui_logger::init_logger(log::LevelFilter::Trace).unwrap();
    tui_logger::set_default_level(log::LevelFilter::Trace);
// env_logger::init();
    let options = PlayerOptions::parse();

    let recording = Recording::try_from(options.file.as_str());

    if let Ok(recording) = recording {
        let (cmd_sender, cmd_receiver) = mpsc::channel();
        let (event_sender, event_receiver) = mpsc::channel();

        // Spawn thread for the Player
        let player_handle = Player::builder()
            .recording(recording)
            .destination(options.destination)
            .source_port(options.source_port)
            .ttl(options.ttl)
            .cmd_rx(cmd_receiver)
            .event_tx(event_sender)
            .build().expect("Failed to initialise Player.");

        let input_handler = input::InputHandler::new(250);

        // Start the tui
        if let Err(error) = tui::run_tui(options, event_receiver, cmd_sender, input_handler) {
            error!("{:?}", error);
            exit(ERROR_RUNTIME);
        }

        player_handle.join().expect("Could not join on the Player thread.");
    } else {
        let error = recording.unwrap_err();
        error!("Cannot play recording, because: {:?}", error);
        exit(ERROR_CANNOT_START);
    };
}
