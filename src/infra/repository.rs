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
            "INSERT INTO pomodoro (id, start_time, end_time, duration, note) VALUES (?, ?, ?, ?, ?)
            ON CONFLICT (id) DO UPDATE SET start_time = excluded.start_time, end_time = excluded.end_time, duration = excluded.duration, note = excluded.note, updated_at = CURRENT_TIMESTAMP
                  ",
            (pomodoro.id().to_string(), pomodoro.start_time().to_rfc3339(), pomodoro.end_time(), pomodoro.duration(), pomodoro.note()),
        ).unwrap();
        println!("Saving pomodoro with id {}", pomodoro.id());
    }
}
