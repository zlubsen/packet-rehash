#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::{Arc, mpsc, Mutex, RwLock};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;
use serde::{Serialize};
use tauri::{Manager, State, WindowEvent};
use tauri::FileDropEvent::Dropped;
use packet_play::{Command, defaults, Event, Player, PlayerState, PositionChange, Recording};

#[derive(thiserror::Error, Debug, Serialize)]
enum PlayError {
    #[error("Error loading file.")]
    CannotLoadFile,
    #[error("Incorrect player state for command {0}.")]
    IncorrectStateForCommand(String),
    #[error("Error updating settings.")]
    UpdateSettingsError
}

struct SettingsWrapper {
    settings: RwLock<Settings>
}

struct Settings {
    file: Option<String>,
    destination: SocketAddr,
    source_port: u16,
    ttl: u32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            file: None,
            destination: SocketAddr::new(
                IpAddr::V4(Ipv4Addr::BROADCAST),
                defaults::DEFAULT_DEST_PORT),
            source_port: defaults::DEFAULT_SRC_PORT,
            ttl: defaults::DEFAULT_TTL
        }
    }
}

struct PlayerWrapper {
    player: RwLock<Option<PlayerHandle>>
}

#[derive(Debug)]
struct PlayerHandle {
    player_handle: JoinHandle<()>,
    current_state : PlayerState,
    current_position : PositionChange,
    cmd_sender: Arc<Mutex<Sender<Command>>>,
    event_receiver: Arc<Mutex<Receiver<Event>>>,
}

impl PlayerHandle {
    fn load_player(recording: Recording, settings: &Settings) -> Self {
        let (cmd_sender, cmd_receiver) = mpsc::channel();
        let (event_sender, event_receiver) = mpsc::channel();

        let player_handle = Player::builder()
            .recording(recording)
            .destination(settings.destination)
            .source_port(settings.source_port)
            .ttl(settings.ttl)
            .cmd_rx(cmd_receiver)
            .event_tx(event_sender)
            .build().unwrap();

        Self {
            player_handle,
            current_state: PlayerState::Initial,
            current_position: Default::default(),
            cmd_sender: Arc::new(Mutex::new(cmd_sender)),
            event_receiver: Arc::new(Mutex::new(event_receiver)),
        }
    }
}

fn main() {
    tauri::Builder::default()
        .manage(SettingsWrapper { settings: RwLock::new(Settings::default()) })
        .manage(PlayerWrapper { player: RwLock::new(None) })
        // .setup(|app| {
        //     let _main_window = app.get_window("main").unwrap();
        //
        //     Ok(())
        // })
        .on_window_event(|event| {
            match event.event() {
                WindowEvent::CloseRequested { .. } => {
                    let player_state: State<PlayerWrapper> = event.window().state();
                    let handle = player_state.player.read().unwrap();
                    if let Some(handle) = &*handle {
                        let _ = &handle.cmd_sender.lock().unwrap().send(Command::Quit);
                        while let Ok(player_event) = &handle.event_receiver.lock().unwrap().recv() {
                            match player_event {
                                Event::QuitCommanded => { return; }
                                _ => {}
                            }
                        }
                    }
                }
                WindowEvent::FileDrop(drop) => {
                    if let Dropped(files) = drop {
                        let file = files.first().unwrap();
                        dbg!(file);
                        dbg!(event);
                        // open_file(event.window, e)
                    }
                    // TODO load file (refactor fn from cmd_open)
                }
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![
            cmd_update_settings,
            cmd_open, cmd_play, cmd_pause, cmd_rewind])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn run_event_thread(receiver: Arc<Mutex<Receiver<Event>>>, window: tauri::Window) -> JoinHandle<()> {
    thread::spawn(move || {
        loop {
            let receiver_lock = receiver.lock().unwrap();
            if let Ok(event) = receiver_lock.recv() {
                match event {
                    Event::Error(err) => {
                        let _ = window.emit("player_event_error", err).unwrap();
                    }
                    Event::PlayerReady => {
                        let _ = window.emit("player_event_ready", "").unwrap();
                    }
                    Event::PlayerStateChanged(state_update) => {
                        let _ = window.emit("player_event_state", state_update).unwrap();
                    }
                    Event::PlayerPositionChanged(position_update) => {
                        let _ = window.emit("player_event_position", position_update).unwrap();
                    }
                    Event::QuitCommanded => {
                        let _ = window.emit("player_event_quit", "").unwrap();
                        return;
                    }
                }
            } else {
                return;
            }
        }
    })
}

fn open_file(window: tauri::Window,
             settings_state: tauri::State<SettingsWrapper>,
             player_state: tauri::State<PlayerWrapper>,
             file_path: &str) -> Result<(), PlayError> {
    match Recording::try_from(file_path) {
        Ok(recording) => {
            {
                let handle = player_state.player.read().unwrap();
                if let Some(handle) = &*handle {
                    let _ = handle.cmd_sender.lock().unwrap().send(Command::Quit);
                    // TODO join on the thread... but join takes ownership and that is not possible behind a shared reference
                    // handle.player_handle.join().expect("Player thread panicked while loading new file.");
                }
            }
            { // update Settings
                let mut settings = settings_state.settings.write().unwrap();
                settings.file = Some(file_path.to_string());
            }

            let player_handle = {
                let settings = settings_state.settings.read().unwrap();
                PlayerHandle::load_player(
                    recording, &*settings)
            };
            { // set new PlayerHandle
                let mut new_handle = player_state.player.write().unwrap();
                *new_handle = Some(player_handle);
            }
            let _event_handle = {
                let state_guard = player_state.player.read().unwrap();
                let receiver = (state_guard.as_ref().unwrap()).event_receiver.clone();
                run_event_thread(receiver, window);
            };

            Ok(())
        }
        Err(_err) => {
            return Err(PlayError::CannotLoadFile)
        }
    }
}

#[tauri::command]
fn cmd_update_settings(settings_state: tauri::State<SettingsWrapper>,
                       destination: &str, source_port: u16, ttl: u32) -> Result<(), PlayError> {
    let mut settings = settings_state.settings.write().unwrap();
    *settings = Settings {
        file: settings.file.clone(),
        destination: destination.parse().expect("Failed to parse socket address"),
        source_port,
        ttl
    };
    Ok(())
}

#[tauri::command]
fn cmd_open(window: tauri::Window,
            settings_state: tauri::State<SettingsWrapper>,
            player_state: tauri::State<PlayerWrapper>,
            file_path: &str) -> Result<(), PlayError> {
    open_file(window, settings_state, player_state, file_path)
}

#[tauri::command]
fn cmd_play(player_state: tauri::State<PlayerWrapper>) -> Result<(), PlayError> {
    let handle = player_state.player.read().unwrap();
    if let Some(handle) = &*handle {
        let _ = &handle.cmd_sender.lock().unwrap().send(Command::Play);
        Ok(())
    } else {
        Err(PlayError::IncorrectStateForCommand(format!("{}", Command::Play)))
    }
}

#[tauri::command]
fn cmd_pause(player_state: tauri::State<PlayerWrapper>) -> Result<(), PlayError> {
    let handle = player_state.player.read().unwrap();
    if let Some(handle) = &*handle {
        let _ = &handle.cmd_sender.lock().unwrap().send(Command::Pause);
        Ok(())
    } else {
        Err(PlayError::IncorrectStateForCommand(format!("{}", Command::Pause)))
    }
}

#[tauri::command]
fn cmd_rewind(player_state: tauri::State<PlayerWrapper>) -> Result<(), PlayError> {
    let handle = player_state.player.read().unwrap();
    if let Some(handle) = &*handle {
        let _ = &handle.cmd_sender.lock().unwrap().send(Command::Rewind);
        Ok(())
    } else {
        Err(PlayError::IncorrectStateForCommand(format!("{}", Command::Rewind)))
    }
}