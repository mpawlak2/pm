use chrono::{Date, DateTime, Utc};


#[derive(Debug)]
pub struct Pomodoro {
    start_time: DateTime<Utc>
}

impl Pomodoro {
    pub fn new() -> Pomodoro {
        Pomodoro {
            start_time: Utc::now(),
        }
    }

    pub fn start_time(&self) -> DateTime<Utc> {
        self.start_time
    }
}
