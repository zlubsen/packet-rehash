#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::process::exit;
use std::sync::mpsc;
use serde_json::Value;
use packet_play::{FileError, Player, PlayerOptions, Recording};

struct State {

}

fn main() {
    run_frontend()
}

fn run_frontend() {
    tauri::Builder::default()
        .setup(|app| {
            let options = match app.get_cli_matches() {
                Ok(matches) => {
                    // TODO emit errors to frontend at startup and close (with Ok button on a modal)
                    let options = if let Some(file) = matches.args.get("file") {
                        if let Value::String(value) = &file.value {
                            PlayerOptions::new(value.clone())
                        } else {
                            println!("file value is not a Value::String");
                            exit(2) }
                    } else {
                        println!("file value not provided");
                        exit(1) };

                    let options = if let Some(destination) = matches.args.get("destination") {
                        if let Value::String(value) = &destination.value {
                            options.with_destination(value.parse().expect("error parsing `destination` argument"))
                        } else {options}
                    } else {options};
                    let options = if let Some(source_port) = matches.args.get("source") {
                        if let Value::String(value) = &source_port.value {
                            options.with_source_port(value.parse().expect("error parsing `source` argument"))
                        } else {options}
                    } else {options};
                    let options = if let Some(ttl) = matches.args.get("ttl") {
                        if let Value::String(value) = &ttl.value {
                            options.with_ttl(value.parse().expect("error parsing `ttl` argument"))
                        } else {options}
                    } else {options};
                    options
                }
                Err(err) => {
                    println!("{err}");
                    exit(1)
                }
            };

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
                    .build().expect("Failed to initialise Player.");
            } else { exit(3) }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![action_play, action_pause, action_rewind])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn action_play() {
    println!("play!");
}

#[tauri::command]
fn action_pause() {
    println!("pause!");
}

#[tauri::command]
fn action_rewind() {
    println!("rewind!");
}
