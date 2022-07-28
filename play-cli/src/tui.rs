use std::io::{stdout, Stdout};
use std::sync::mpsc::{Receiver, Sender, TryRecvError};

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use log::trace;
use tui::backend::{Backend, CrosstermBackend};
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::{Frame, Terminal};
use tui::layout::Alignment::Center;
use tui::layout::Direction::Horizontal;
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph};
use tui_logger::TuiLoggerWidget;

use packet_play::{Event, Command, PlayerOptions, PlayerError, PlayerState, PositionChange};
use crate::actions::Action;

use crate::input::{Input, InputHandler};

const BUTTONS : [&str; 4] = ["Play", "Pause", "Rewind", "Quit"];

struct App {
    options: PlayerOptions,
    current_state : PlayerState,
    current_position : PositionChange,
    cmd_sender : Sender<Command>,
    event_receiver: Receiver<Event>,
    input_handler : InputHandler,
    kill_signal : bool,
    selected_button : usize,
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
            trace!("sending {command:?}");
            self.cmd_sender.send(command).expect("Failed to send command");
        }
    }

    fn handle_quit(&mut self) -> Option<Command> {
        self.kill_signal = true;
        Some(Command::Quit)
    }

    fn handle_left(&mut self) -> Option<Command> {
        self.selected_button = self.selected_button.saturating_sub(1);
        trace!("left > {}", self.selected_button);
        None
    }

    fn handle_right(&mut self) -> Option<Command> {
        if self.selected_button < BUTTONS.len() - 1 {
             self.selected_button = self.selected_button + 1
        };
        trace!("right > {}", self.selected_button);
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
        if self.current_state == PlayerState::Initial ||
            self.current_state == PlayerState::Paused {
            Some(Command::Play)
        } else if self.current_state == PlayerState::Playing {
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
        options,
        current_state: PlayerState::Initial,
        current_position: Default::default(),
        cmd_sender,
        event_receiver,
        input_handler,
        kill_signal: false,
        selected_button: 0,
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
        if let Err(_) = terminal.draw(|rect| draw(rect, &mut app)) {
            app.kill_signal = true;
        }

        match app.event_receiver.try_recv() {
            Ok(event) => {
                match event {
                    Event::Error(_) => {}
                    Event::PlayerReady => {}
                    Event::PlayerStateChanged(new_state) => {
                        app.current_state = new_state.state;
                    }
                    Event::PlayerPositionChanged(new_position) => {
                        app.current_position = new_position;
                    }
                    Event::QuitCommanded => {
                        app.kill_signal = true;
                    }
                }
            }
            Err(e) => {
                match e {
                    TryRecvError::Empty => {}
                    TryRecvError::Disconnected => { app.kill_signal = true; }
                }
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
        app: &mut App)
where B: Backend {
    let size = rect.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(2),
                Constraint::Length(3),
                Constraint::Length(10),
            ].as_ref(),
        )
        .split(size);

    let header = Paragraph::new(vec![Spans::from(Span::raw("Packet Play")), Spans::from(Span::raw(app.options.file.as_str()))])
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    // let filename = Paragraph::new(options.file.as_str())
    //     .style(Style::default().fg(Color::White))
    //     .alignment(Alignment::Center)
    //     .block(Block::default().borders(Borders::ALL));
    rect.render_widget(header, chunks[0]);

    // let content_placeholder = Block::default().borders(Borders::ALL);
    let content_placeholder = Paragraph::new(format!("{:?}",app.current_state));
    rect.render_widget(content_placeholder, chunks[1]);

    let buttons_chunks = Layout::default()
        .direction(Horizontal)
        .margin(2)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ].as_ref(),)
        .split(chunks[2]);
    // for (i, button) in BUTTONS.iter().enumerate() {
    //     let paragraph = Paragraph::new(*button).alignment(Center).style(
    //         if app.selected_button == i {
    //             Style::default().fg(Color::White)
    //         } else {
    //             Style::default().fg(Color::Blue).bg(Color::Gray)
    //         }
    //     );
    //     rect.render_widget(paragraph, buttons_chunks[i]);
    // }

    let buttons = BUTTONS.iter().enumerate().map(|(i, btn_text)| {
        (i, Paragraph::new(*btn_text).alignment(Center).style(
            if app.selected_button == i {
                Style::default().fg(Color::White)
            } else {
                Style::default().fg(Color::Blue).bg(Color::Gray)
            }
        ))
    }).for_each(|(i, btn)| rect.render_widget(btn, buttons_chunks[i]));

    let logs = draw_logs();
    rect.render_widget(logs, chunks[3]);
}

fn draw_logs<'a>() -> TuiLoggerWidget<'a> {
    TuiLoggerWidget::default()
        .style_error(Style::default().fg(Color::Red))
        .style_debug(Style::default().fg(Color::Green))
        .style_warn(Style::default().fg(Color::Yellow))
        .style_trace(Style::default().fg(Color::Gray))
        .style_info(Style::default().fg(Color::Blue))
        .block(
            Block::default()
                .title("Messages")
                .border_style(Style::default().fg(Color::White).bg(Color::Black))
                .borders(Borders::ALL),
        )
        .style(Style::default().fg(Color::White).bg(Color::Black))
}