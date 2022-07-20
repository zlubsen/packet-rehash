use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use std::time::Duration;
use eframe::NativeOptions;
use egui::Button;
use log::{trace};
use crate::{PlayerOptions};
use packet_play::{Command, Event, PositionChange};
use packet_play::{PlayerError, PlayerState, PLAYER_STARTUP_TIMEOUT_MS};
use packet_rehash_core::utils::format::FormattedDuration;

pub(crate) fn run_gui(options: PlayerOptions, event_receiver: Receiver<Event>, cmd_sender: Sender<Command>) -> Result<(), PlayerError> {
    // // TODO move channel, player creation except gui to main.
    // let (cmd_sender, cmd_receiver) = mpsc::channel();
    // let (event_sender, event_receiver) = mpsc::channel();
    //
    // // Spawn thread for the Player
    // let _player_handle = Player::builder()
    //     .recording(recording)
    //     .destination(options.destination)
    //     .source_port(options.source_port)
    //     .ttl(options.ttl)
    //     .cmd_rx(cmd_receiver)
    //     .event_tx(event_sender)
    //     .build()?;

    // Wait for Player to be initialised
    loop {
        match event_receiver.recv_timeout(Duration::from_secs(PLAYER_STARTUP_TIMEOUT_MS)) {
            Ok(event) => {
                match event {
                    Event::Error(err) => { return Err(err); }
                    Event::PlayerReady => {
                        break; }
                    _ => { trace!("Unexpected to see this event here..."); }
                }
            }
            Err(_err) => {
                return Err(PlayerError::PlayerInitError);
            }
        }
    }

    if !options.auto_play_disable {
        let _ = cmd_sender.send(Command::Play);
    }

    let window_options = window_options();
    eframe::run_native(
        "packet-play",
        window_options,
        Box::new(|_cc| Box::new(
            GuiApp::new(options, cmd_sender, event_receiver)))
    );
}

struct GuiApp {
    options: PlayerOptions,
    current_state : PlayerState,
    current_position : PositionChange,
    cmd_sender: Sender<Command>,
    event_receiver: Receiver<Event>,
}

impl GuiApp {
    pub fn new(options: PlayerOptions, cmd_sender: Sender<Command>, event_receiver: Receiver<Event>) -> Self {
        Self {
            options,
            current_state: PlayerState::Initial,
            current_position: Default::default(),
            cmd_sender,
            event_receiver,
        }
    }
}

impl eframe::App for GuiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let message : Option<String> = match self.event_receiver.try_recv() {
            Ok(Event::QuitCommanded) => { None } // Not needed with GUI
            Ok(Event::PlayerReady) => { None } // Already passed
            Ok(Event::PlayerStateChanged(state)) => {
                self.current_state = state.state;
                None
            }
            Ok(Event::PlayerPositionChanged(position)) => {
                self.current_position = position;
                None
            }
            Ok(Event::Error(error)) => { Some(format!("{error:?}")) }
            Err(TryRecvError::Empty) => { None }
            Err(TryRecvError::Disconnected) => {
                Some("Event channel disconnected, Player stopped working. Exiting.".to_string())
            }
        };

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Packet Play");
            ui.label(format!("Recording: {}", self.options.file));

            ui.horizontal(|ui| {
                ui.label(self.current_state.to_string());
                ui.add(egui::ProgressBar::new(
                    self.current_position.position as f32 / self.current_position.max_position as f32)
                    .animate(self.current_state == PlayerState::Playing));

            });
            ui.label(format!("Packets: [{}/{}]", self.current_position.position, self.current_position.max_position));
            ui.label(format!("Time: [ {} / {} ]", FormattedDuration::new(self.current_position.time_position), FormattedDuration::new(self.current_position.time_total)));

            ui.horizontal(|ui| {
                if ui.add_enabled(
                    self.current_state != PlayerState::Playing,
                    Button::new("Play")).clicked() {
                    let _ = self.cmd_sender.send(Command::Play);
                }
                if ui.add_enabled(
                    self.current_state == PlayerState::Playing,
                    Button::new("Pause")).clicked() {
                    let _ = self.cmd_sender.send(Command::Pause);
                }
                if ui.add_enabled(
                    self.current_state != PlayerState::Initial,
                    Button::new("Rewind")).clicked() {
                    let _ = self.cmd_sender.send(Command::Rewind);
                }
            });
            ui.label(message.unwrap_or("".to_string()));
        });
    }

    fn on_exit(&mut self, _gl: &eframe::glow::Context) {
        // Send Quit command to Player
        let _ = self.cmd_sender.send(Command::Quit);
        // Wait for player to shutdown
        loop {
            if let Ok(Event::QuitCommanded) = self.event_receiver.try_recv() {
                break;
            }
        }
    }
}

fn window_options() -> NativeOptions {
    NativeOptions {
        decorated: true,
        initial_window_size: Some(egui::Vec2::new(500f32,150f32)),
        ..Default::default()
    }
}