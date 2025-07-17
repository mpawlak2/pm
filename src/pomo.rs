use std::fmt;

use chrono::{Date, DateTime, TimeDelta, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PomodoroId(Uuid);

impl PomodoroId {
    fn new() -> PomodoroId {
        PomodoroId(Uuid::new_v4())
    }
}

impl fmt::Display for PomodoroId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub struct Pomodoro {
    pub id: PomodoroId,
    start_time: DateTime<Utc>,
    end_time: Option<DateTime<Utc>>,
    duration: TimeDelta,
    note: Option<String>,
}

impl Pomodoro {
    pub fn new() -> Pomodoro {
        let default_duration = 25;
        Pomodoro {
            id: PomodoroId::new(),
            start_time: Utc::now(),
            end_time: None,
            duration: TimeDelta::minutes(default_duration),
            note: None,
        }
    }

    pub fn with_minutes(mut self, value: i64) -> Self {
        self.duration = TimeDelta::minutes(value);
        self
    }

    pub fn start_time(&self) -> DateTime<Utc> {
        self.start_time
    }

    pub fn is_time_up(&self) -> bool {
        self.start_time + self.duration < Utc::now()
    }

    pub fn finish(&mut self, note: &str) {
        self.end_time = Some(Utc::now());
        self.note = Some(note.to_string());
    }
}
