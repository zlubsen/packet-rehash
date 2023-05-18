use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Command {
    Play,
    Pause,
    Rewind,
    Quit,
    Seek(usize),
}

impl Command {
    pub fn as_vec() -> Vec<&'static str> {
        vec![
            "Play",
            "Pause",
            "Rewind",
            "Quit",
            "Seek",
        ]
    }
}

impl From<usize> for Command {
    fn from(value: usize) -> Self {
        match value {
            0 => { Command::Play }
            1 => { Command::Pause }
            2 => { Command::Rewind }
            3 => { Command::Seek(0) }
            4 | _ => { Command::Quit }
        }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Play => { write!(f, "Play") }
            Command::Pause => { write!(f, "Pause") }
            Command::Rewind => { write!(f, "Rewind") }
            Command::Quit => { write!(f, "Quit") }
            Command::Seek(_) => { write!(f, "Seek") }
        }
    }
}
