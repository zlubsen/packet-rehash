use crate::input::Key;
use crossterm::event;
use std::{sync::mpsc, thread, time::Duration};

const TICK_RATE: u64 = 200;

#[derive(Debug, Clone, Copy)]
/// Configuration for event handling.
pub struct InputConfig {
    /// The key that is used to exit the application.
    pub exit_key: Key,
    /// The tick rate at which the application will sent an tick event.
    pub tick_rate: Duration,
}

impl Default for InputConfig {
    fn default() -> Self {
        Self {
            exit_key: Key::Ctrl('c'),
            tick_rate: Duration::from_millis(TICK_RATE),
        }
    }
}

/// An occurred event.
pub enum Input<I> {
    /// An input event occurred.
    Input(I),
    /// An tick event occurred.
    Tick,
}

/// A small event handler that wrap crossterm input and tick event. Each event
/// type is handled in its own thread and returned to a common `Receiver`
pub struct InputHandler {
    rx: mpsc::Receiver<Input<Key>>,
    // Need to be kept around to prevent disposing the sender side.
    _tx: mpsc::Sender<Input<Key>>,
}

impl InputHandler {
    /// Constructs an new instance of `Events` with the default config.
    pub fn new(tick_rate: u64) -> Self {
        InputHandler::with_config(InputConfig {
            tick_rate: Duration::from_millis(tick_rate),
            ..Default::default()
        })
    }

    /// Constructs an new instance of `Events` from given config.
    pub fn with_config(config: InputConfig) -> Self {
        let (tx, rx) = mpsc::channel();

        let input_tx = tx.clone();
        thread::spawn(move || {
            loop {
                // poll for tick rate duration, if no event, sent tick event.
                if event::poll(config.tick_rate).unwrap() {
                    if let event::Event::Key(key) = event::read().unwrap() {
                        let key = Key::from(key);

                        input_tx.send(Input::Input(key)).unwrap();
                    }
                } else {
                    // send tick, and the handler should quit when the input channel closes due to the receiver shutting down
                    let should_quit = input_tx.send(Input::Tick).map_or_else(|_| true, |()| false);
                    if should_quit {
                        break;
                    }
                }
            }
        });

        Self { rx, _tx: tx }
    }

    /// Attempts to read an event.
    /// This function will block the current thread.
    pub fn next(&self) -> Result<Input<Key>, mpsc::RecvError> {
        self.rx.recv()
    }

    pub fn try_next(&self) -> Result<Input<Key>, mpsc::TryRecvError> {
        self.rx.try_recv()
    }
}