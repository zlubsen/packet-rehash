mod tui;
pub mod input;

use std::env;
use std::fs::File;
use std::process::exit;
use std::sync::mpsc;

use input::start_input_thread;

use clap::Parser;
use packet_play::{Player, PlayerOptions, Recording};

fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }

    let options = PlayerOptions::parse();

    let recording = Recording::try_from(options.file.as_str());

    if let Ok(recording) = recording {
        let (input_sender, input_receiver) = mpsc::channel();
        let (cmd_sender, cmd_receiver) = mpsc::channel();
        let (event_sender, event_receiver) = mpsc::channel();

        // Spawn thread for the Player
        let _player_handle = Player::builder()
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
    } else {
        let error = recording.unwrap_err();
        error!("Cannot play recording, because: {:?}", error);
        exit(ERROR_CANNOT_START);
    };
}
