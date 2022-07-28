use std::time::Duration;
use crate::player::PlayerState;
use crate::PlayerError;

#[derive(Clone)]
pub enum Event {
    Error(PlayerError),
    PlayerReady,
    PlayerStateChanged(StateChange),
    PlayerPositionChanged(PositionChange),
    QuitCommanded,
}

impl Event {
    pub(crate) fn state_event(state: PlayerState) -> Self {
        Event::PlayerStateChanged(StateChange{
            state
        })
    }

    pub(crate) fn position_event(current_pos: usize, max_pos: usize, current_time:Duration, total_time: Duration) -> Self {
        // Note: This function increases the position with +1 to compensate for 0-based vec indexing.
        Event::PlayerPositionChanged(PositionChange{
            position: current_pos + 1,
            max_position: max_pos,
            time_position: current_time,
            time_total: total_time,
        })
    }

    pub(crate) fn error(error: PlayerError) -> Self {
        Event::Error(error)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct StateChange {
    pub state: PlayerState,
}

#[derive(Copy, Clone, Debug)]
pub struct PositionChange {
    pub position: usize,
    pub max_position: usize,
    pub time_position: Duration,
    pub time_total: Duration,
}

impl Default for PositionChange {
    fn default() -> Self {
        Self {
            position: 0,
            max_position: 0,
            time_position: Duration::from_secs(0),
            time_total: Duration::from_secs(0),
        }
    }
}