use crate::pomo;

pub struct PomodoroRepository<'a> {
    database_connection: &'a rusqlite::Connection,
}

impl<'a> PomodoroRepository<'a> {
    pub fn new(database_connection: &rusqlite::Connection) -> PomodoroRepository {
        PomodoroRepository {
            database_connection: database_connection,
        }
    }

    pub fn save(&self, pomodoro: &pomo::Pomodoro) {
        println!("Saving pomodoro with id {}", pomodoro.id());
    }
}
