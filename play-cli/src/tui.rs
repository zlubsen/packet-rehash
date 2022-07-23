use std::io::{stdout, Stdout};
use std::sync::mpsc::{Receiver, Sender};

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::Terminal;
use tui::widgets::{Block, Borders, Paragraph};

use packet_play::{Event, Command, PlayerOptions, PlayerError, PlayerState};
use crate::actions::Action;

use crate::input::{Input, InputHandler, Key};

struct App {
    kill_signal : bool,
    event_receiver: Receiver<Event>,
    cmd_sender : Sender<Command>,
    input_handler : InputHandler,
    selected_button : usize,
    state : PlayerState,
}

impl App {
    pub fn handle_action(&mut self, action : Action) {
        let command = match action {
            Action::MoveLeft => { self.handle_left() }
            Action::MoveRight => { self.handle_right() }
            Action::MoveUp => { None }
            Action::MoveDown => { None }
            Action::Enter => { self.handle_enter() }
            Action::Quit => { self.handle_quit() }
            Action::Play => { self.handle_play() }
            Action::Pause => { self.handle_pause() }
            Action::Rewind => { self.handle_rewind() }
        };
        if let Some(command) = command {
            self.cmd_sender.send(command).expect("Failed to send command");
        }
    }

    fn handle_quit(&mut self) -> Option<Command> {
        self.kill_signal = true;
        Some(Command::Quit)
    }

    fn handle_play(&self) -> Option<Command> {
        if self.state == PlayerState::Initial ||
            self.state == PlayerState::Paused {
            Some(Command::Play)
        } else { None }
    }

    fn handle_pause(&self) -> Option<Command> {
        if self.state == PlayerState::Playing ||
            self.state == PlayerState::Paused {
            Some(Command::Pause)
        } else { None }
    }

    fn handle_rewind(&self) -> Option<Command> {
        Some(Command::Rewind)
    }
}

pub(crate) fn run_tui(options: PlayerOptions, event_receiver: Receiver<Event>, cmd_sender: Sender<Command>, input_handler: InputHandler) -> Result<(), PlayerError> {
    enable_raw_mode().expect("Failed to set raw mode");
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen).expect("Failed to switch to alternate screen");
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
    execute!(terminal.backend_mut(), LeaveAlternateScreen).expect("Failed to return from alternate screen");
    terminal.show_cursor().expect("Terminal failed to show cursor.");
    Ok(())
}

fn gui_loop(mut app: App,
            mut terminal: Terminal<CrosstermBackend<Stdout>>)
    -> Terminal<CrosstermBackend<Stdout>> {
    while !app.kill_signal {
        if let Err(_) = terminal.draw(|rect| {
            let size = rect.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(2),
                        Constraint::Length(6),
                    ].as_ref(),
                )
                .split(size);

            let header = Paragraph::new("Packet Play")
                .style(Style::default().fg(Color::White))
                .block(Block::default().borders(Borders::ALL));
            rect.render_widget(header, chunks[0]);
        }) {
            app.kill_signal = true;
        }

        if let Ok(event) = app.event_receiver.try_recv() {
            match event {
                Event::Error(_) => {}
                Event::PlayerReady => {}
                Event::PlayerStateChanged(_) => {}
                Event::PlayerPositionChanged(_) => {}
                Event::QuitCommanded => {}
            }
        }

        if let Ok(input) = app.input_handler.next() {
            match input {
                Input::Input(key) => {
                    match key {
                        Key::Enter => {}
                        Key::Tab => {}
                        Key::Backspace => {}
                        Key::Esc => {}
                        Key::Left => {}
                        Key::Right => {}
                        Key::Up => {}
                        Key::Down => {}
                        Key::Ins => {}
                        Key::Delete => {}
                        Key::Home => {}
                        Key::End => {}
                        Key::PageUp => {}
                        Key::PageDown => {}
                        Key::F0 => {}
                        Key::F1 => {}
                        Key::F2 => {}
                        Key::F3 => {}
                        Key::F4 => {}
                        Key::F5 => {}
                        Key::F6 => {}
                        Key::F7 => {}
                        Key::F8 => {}
                        Key::F9 => {}
                        Key::F10 => {}
                        Key::F11 => {}
                        Key::F12 => {}
                        Key::Char(_) => {}
                        Key::Ctrl(char) => {}
                        Key::Alt(_) => {}
                        Key::Unknown => {}
                    }
                }
                Input::Tick => {}
            }
        }
    }
    terminal
}