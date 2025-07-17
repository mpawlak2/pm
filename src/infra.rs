use rusqlite::Connection;

use super::pomo;

pub struct PomodoroRepository;

impl PomodoroRepository {
    pub fn new() -> PomodoroRepository {
        PomodoroRepository {}
    }

    pub fn save(&self, pomodoro: &pomo::Pomodoro) {
        println!("Saving pomodoro with id {}", pomodoro.id());
    }
}

pub fn create_database_connection(path: &str) -> Connection {
    Connection::open(path).expect("Error initializing database connection.")
}
