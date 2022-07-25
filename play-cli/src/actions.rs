use std::fmt::{Display, Formatter};
use std::slice::Iter;
use crate::input::{Key};

#[derive(Debug, Clone, Copy)]
pub enum Action {
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    CycleArea,
    Enter,
    Quit,
    PlayPause,
    Rewind,
}

impl Action {
    pub fn iterator() -> Iter<'static, Action> {
        static ACTIONS : [Action; 9] = [
            Action::MoveLeft,
            Action::MoveRight,
            Action::MoveUp,
            Action::MoveDown,
            Action::CycleArea,
            Action::Enter,
            Action::Quit,
            Action::PlayPause,
            Action::Rewind,
        ];
        ACTIONS.iter()
    }

    pub fn key_mapping(&self) -> &[Key] {
        match self {
            Action::MoveLeft => &[Key::Left],
            Action::MoveRight => &[Key::Right],
            Action::MoveUp => &[Key::Up],
            Action::MoveDown => &[Key::Down],
            Action::CycleArea => &[Key::Tab],
            Action::Enter => &[Key::Enter],
            Action::Quit => &[Key::Ctrl('c'), Key::Char('q')],
            Action::PlayPause => &[Key::Char('p'), Key::Char(' ')],
            Action::Rewind => &[Key::Char('r')],
        }
    }

    pub fn from_key(key: &Key) -> Option<Self> {
        Action::iterator()
            .find(|action| action.key_mapping().contains(&key))
            .and_then(|action|Some(action.clone()))
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Action::MoveLeft => "Left",
            Action::MoveRight => "Right",
            Action::MoveUp => "Up",
            Action::MoveDown => "Down",
            Action::CycleArea => "Change Area",
            Action::Enter => "Enter Selection",
            Action::Quit => "Quit",
            Action::PlayPause => "Play/Pause",
            Action::Rewind => "Rewind",
        };
        write!(f, "{text}")
    }
}