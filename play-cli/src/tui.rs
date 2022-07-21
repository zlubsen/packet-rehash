use std::io::{stdout, Stdout};
use std::sync::mpsc::{Receiver, Sender};
use crossterm::terminal::disable_raw_mode;
use tui::backend::CrosstermBackend;
use tui::Terminal;
use packet_play::{Event, Command, PlayerOptions, PlayerError};
use crate::input::InputHandler;

struct App {
    kill_signal : bool,
    event_receiver: Receiver<Event>,
    cmd_sender : Sender<Command>,
    input_handler : InputHandler,
}

pub(crate) fn run_tui(options: PlayerOptions, event_receiver: Receiver<Event>, cmd_sender: Sender<Command>, input_handler: InputHandler) -> Result<(), PlayerError> {
    let stdout = stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).expect("Failed to create terminal.");
    terminal.clear().expect("Failed to clear terminal.");

    let app = App {
        kill_signal: false,
        event_receiver,
        cmd_sender,
        input_handler,
    };

    let mut terminal = gui_loop(app, terminal);

    disable_raw_mode().expect("Failed to disable raw mode.");
    terminal.show_cursor().expect("Terminal failed to show cursor.");
    terminal.clear().expect("Failed to clear terminal.");
}

fn gui_loop(mut app: App,
            mut terminal: Terminal<CrosstermBackend<Stdout>>)
    -> Terminal<CrosstermBackend<Stdout>> {
    while !app.kill_signal {

    }
    terminal
}