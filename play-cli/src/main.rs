mod tui;

use std::env;
use std::fs::File;
use std::process::exit;
use std::sync::mpsc;

use clap::Parser;

fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }

    let options = PlayerOptions::parse();

    let recording = Recording::try_from(options.file.as_str());

    if let Ok(recording) = recording {
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
            .build().expect("Player init error...");

        // Start the tui
        // if let Err(error) = gui::run_gui(options, event_receiver, cmd_sender) {
        //     error!("{:?}", error);
        //     exit(ERROR_RUNTIME);
        // }
    } else {
        let error = recording.unwrap_err();
        error!("Cannot play recording, because: {:?}", error);
        exit(ERROR_CANNOT_START);
    };
}
