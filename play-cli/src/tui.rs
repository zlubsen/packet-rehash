use std::fmt::format;
use std::io::{stdout, Stdout};
use std::sync::mpsc::{Receiver, Sender};

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::backend::{Backend, CrosstermBackend};
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::{Frame, Terminal};
use tui::widgets::{Block, Borders, Paragraph};

use packet_play::{Event, Command, PlayerOptions, PlayerError, PlayerState};
use crate::actions::Action;

use crate::input::{Input, InputHandler};

const BUTTONS : [&str; 4] = ["Play", "Pause", "Rewind", "Quit"];

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
            Action::PlayPause => { self.handle_playpause() }
            Action::Rewind => { self.handle_rewind() }
            Action::CycleArea => { self.handle_cycle_area() }
        };
        if let Some(command) = command {
            self.cmd_sender.send(command).expect("Failed to send command");
        }
    }

    fn handle_quit(&mut self) -> Option<Command> {
        self.kill_signal = true;
        Some(Command::Quit)
    }

    fn handle_left(&mut self) -> Option<Command> {
        self.selected_button = self.selected_button.saturating_sub(1);
        None
    }

    fn handle_right(&mut self) -> Option<Command> {
        if self.selected_button == BUTTONS.len() - 1 {
             self.selected_button = self.selected_button + 1
        };

        None
    }

    fn handle_enter(&mut self) -> Option<Command> {
        match self.selected_button {
            0 => Some(Command::Play),
            1 => Some(Command::Pause),
            2 => Some(Command::Rewind),
            3 => Some(Command::Quit),
            _ => None,
        }
    }

    fn handle_playpause(&self) -> Option<Command> {
        if self.state == PlayerState::Initial ||
            self.state == PlayerState::Paused {
            Some(Command::Play)
        } else if self.state == PlayerState::Playing {
            Some(Command::Pause)
        } else { None }
    }

    fn handle_rewind(&self) -> Option<Command> {
        Some(Command::Rewind)
    }

    fn handle_cycle_area(&self) -> Option<Command> {
        // TODO impl 'active area' functions
        None
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
        selected_button: 0,
        state: PlayerState::Initial
    };

    let mut terminal = gui_loop(app, &options, terminal);

    disable_raw_mode().expect("Failed to disable raw mode.");
    execute!(terminal.backend_mut(), LeaveAlternateScreen).expect("Failed to return from alternate screen");
    terminal.show_cursor().expect("Terminal failed to show cursor.");
    Ok(())
}

fn gui_loop(mut app: App,
            options: &PlayerOptions,
            mut terminal: Terminal<CrosstermBackend<Stdout>>)
    -> Terminal<CrosstermBackend<Stdout>> {
    while !app.kill_signal {
        if let Err(_) = terminal.draw(|rect| draw(rect, &mut app, options)) {
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

        if let Ok(input) = app.input_handler.next() { // Blocks, continues on key input or tick
            match input {
                Input::Input(key) => {
                    if let Some(action) = Action::from_key(&key) {
                        app.handle_action(action);
                    }
                }
                Input::Tick => {}
            }
        }
    }
    terminal
}

fn draw<B>(rect: &mut Frame<B>,
        app: &mut App,
        options: &PlayerOptions)
where B: Backend {
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
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    // let filename = Paragraph::new(options.file.as_str())
    //     .style(Style::default().fg(Color::White))
    //     .alignment(Alignment::Center)
    //     .block(Block::default().borders(Borders::ALL));
    rect.render_widget(header, chunks[0]);
}