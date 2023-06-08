use std::io::{stdout, Stdout};
use std::path::Path;
use std::sync::mpsc::{Receiver, Sender};

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::{Frame, Terminal};
use ratatui::text::{Span, Line};
use ratatui::widgets::{Block, Borders, Cell, Gauge, Paragraph, Row, Table};
use tui_logger::TuiLoggerWidget;

use packet_play::{Event, Command, PlayerOptions, PlayerError, PlayerState, PositionChange};
use packet_rehash_core::utils::format::FormattedDuration;
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
            self.cmd_sender.send(command).expect("Failed to send command, receiver (Player) disconnected.");
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
        if self.selected_button < BUTTONS.len() - 1 {
             self.selected_button = self.selected_button + 1
        };
        None
    }

    fn handle_enter(&mut self) -> Option<Command> {
        match self.selected_button {
            0 => Some(Command::Play),
            1 => Some(Command::Pause),
            2 => Some(Command::Rewind),
            3 => { self.kill_signal = true; Some(Command::Quit) },
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
        // Read all events currently in the event channel
        for event in app.event_receiver.try_iter() {
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

        // Blocks to limit draw rate, continues on key input or tick
        if let Ok(input) = app.input_handler.next() {
            match input {
                Input::Input(key) => {
                    if let Some(action) = Action::from_key(&key) {
                        app.handle_action(action);
                    }
                }
                Input::Tick => {
                }
            }
            if let Err(_) = terminal.draw(|frame| draw(frame, &mut app)) {
                app.kill_signal = true;
            }
        }
    }
    terminal
}

fn draw<B>(frame: &mut Frame<B>,
           app: &mut App)
where B: Backend {
    let size = frame.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints(
            [
                Constraint::Length(4),
                Constraint::Min(2),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(10),
            ].as_ref(),
        )
        .split(size);

    let header = draw_header(&app);
    frame.render_widget(header, chunks[0]);

    let info_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(10),
            Constraint::Length(40),
        ].as_ref())
        .split(chunks[1]);

    let recording_info = draw_info(&app);
    frame.render_widget(recording_info, info_chunks[0]);

    let help_info = draw_help();
    frame.render_widget(help_info, info_chunks[1]);

    let progress_bar = draw_progress(&app);
    frame.render_widget(progress_bar, chunks[2]);

    let buttons_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ].as_ref(),)
        .split(chunks[3]);

    let control_block = Block::default()
        .title(
            Span::styled("Controls", Style::default().fg(Color::White)))
        .borders(Borders::ALL);
    frame.render_widget(control_block, chunks[3]);
    BUTTONS.iter().enumerate().map(|(i, btn_text)| {
        (i, Paragraph::new(btn_text.clone()).alignment(Alignment::Center).style(
            if app.selected_button == i {
                Style::default().fg(Color::Blue).bg(Color::White)
            } else {
                Style::default().fg(Color::White)
            }
        ))
    }).for_each(|(i, btn)| frame.render_widget(btn, buttons_chunks[i]));

    let logs = draw_logs();
    frame.render_widget(logs, chunks[4]);
}

fn draw_header(app: &App) -> Paragraph {
    Paragraph::new(
        vec![
            Line::from(Span::styled("Packet Play", Style::default().fg(Color::White))),
            Line::from(Span::from(app.options.file.as_str()))
        ])
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL))
}

fn draw_info(app: &App) -> Table {
    let file_name = if let Some(os_str) = Path::new(&app.options.file).file_name() {
        if let Some(name) = os_str.to_str() {
            name
        } else { "<incorrect OsStr>" }
    } else { "<incorrect path>" };
    let info_key_style = Style::default().fg(Color::Gray);
    let info_value_style = Style::default().fg(Color::White);
    let recording_rows = vec![
        Row::new(vec![
            Cell::from(Span::styled("File:", info_key_style)),
            Cell::from(Span::styled(file_name, info_value_style)),
        ]),
        Row::new(vec![
            Cell::from(Span::styled("Destination:", info_key_style)),
            Cell::from(Span::styled(app.options.destination.to_string(), info_value_style)),
        ]),
        Row::new(vec![
            Cell::from(Span::styled("Source port:", info_key_style)),
            Cell::from(Span::styled(app.options.source_port.to_string(), info_value_style)),
        ]),
        Row::new(vec![
            Cell::from(Span::styled("TTL:", info_key_style)),
            Cell::from(Span::styled(app.options.ttl.to_string(), info_value_style)),
        ]),
        Row::new(vec![
            Cell::from(Span::styled("Auto play:", info_key_style)),
            Cell::from(Span::styled((!app.options.auto_play_disable).to_string(), info_value_style)),
        ]),
        Row::new(vec![
            Cell::from(Span::styled("", info_key_style)),
            Cell::from(Span::styled("", info_value_style)),
        ]),
        Row::new(vec![
            Cell::from(Span::styled("State:", info_key_style)),
            Cell::from(Span::styled(app.current_state.to_string(), info_value_style)),
        ]),
        Row::new(vec![
            Cell::from(Span::styled("Packets:", info_key_style)),
            Cell::from(Span::styled(format!("{} / {}",app.current_position.position, app.current_position.max_position), info_value_style)),
        ]),
    ];

    Table::new(recording_rows)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Player status"),
        )
        .widths(&[Constraint::Length(15), Constraint::Min(20)])
        .column_spacing(1)
}

fn draw_help() -> Table<'static> {
    let help_key_style = Style::default().fg(Color::LightCyan);
    let help_value_style = Style::default().fg(Color::Gray);

    let mut help_rows = vec![];
    for action in Action::iterator() {
        let mut first = true;
        for key in action.key_mapping() {
            let help = if first {
                first = false;
                action.to_string()
            } else {
                String::from("")
            };
            let help_item = Row::new(vec![
                Cell::from(Span::styled(help, help_value_style)),
                Cell::from(Span::styled(key.to_string(), help_key_style)),
            ]);
            help_rows.push(help_item);
        }
    }

    Table::new(help_rows)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Help"),
        )
        .widths(&[Constraint::Length(20), Constraint::Min(20)])
        .column_spacing(1)
}

fn draw_progress(app: &App) -> Gauge {
    let progress_percent = (app.current_position.time_position.as_secs() * 100)
        .checked_div(app.current_position.time_total.as_secs()).unwrap_or(0);

    Gauge::default()
        .block(Block::default().borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Blue))
        .label(
            format!("{progress_percent}% ({} / {})",
                    FormattedDuration::new(app.current_position.time_position),
                    FormattedDuration::new(app.current_position.time_total)))
        .percent(progress_percent as u16)
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
                .title(Span::styled("Messages", Style::default().fg(Color::White)))
                .border_style(Style::default().fg(Color::White).bg(Color::Black))
                .borders(Borders::ALL),
        )
        .style(Style::default().fg(Color::White).bg(Color::Black))
}