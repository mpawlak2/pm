use std::fmt;

use chrono::{DateTime, TimeDelta, Utc};
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
    id: PomodoroId,
    start_time: DateTime<Utc>,
    end_time: Option<DateTime<Utc>>,
    duration: i32,
    note: Option<String>,
}

impl Pomodoro {
    pub fn new() -> Pomodoro {
        let default_duration = 25;
        Pomodoro {
            id: PomodoroId::new(),
            start_time: Utc::now(),
            end_time: None,
            duration: default_duration,
            note: None,
        }
    }

    // Builder lite
    pub fn with_minutes(mut self, value: i32) -> Self {
        self.duration = value;
        self
    }

    // Getters & Properties
    pub fn id(&self) -> &PomodoroId {
        &self.id
    }
    pub fn start_time(&self) -> DateTime<Utc> {
        self.start_time
    }

    pub fn duration(&self) -> i32 {
        self.duration
    }

    pub fn is_done(&self) -> bool {
        self.start_time + TimeDelta::minutes(self.duration as i64) < Utc::now()
    }

    // Methods
    pub fn finish(&mut self, note: &str) {
        self.end_time = Some(Utc::now());
        self.note = Some(note.to_string());
    }
}
