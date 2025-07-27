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
        self.database_connection.execute(
            "INSERT INTO pomodoro (id, start_time, duration) VALUES (?, ?, ?)",
            (pomodoro.id().to_string(), pomodoro.start_time(), pomodoro.duration()),
        ).unwrap();
        println!("Saving pomodoro with id {}", pomodoro.id());
    }
}
