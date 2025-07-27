use std::fmt::Debug;

use chrono::{DateTime, Utc};

use crate::pomo;

#[derive(Debug)]
struct PomodoroDTO {
    id: String,
    start_time: DateTime<Utc>,
    end_time: Option<DateTime<Utc>>,
    duration: i32,
    note: Option<String>,
}

impl PomodoroDTO {
    fn from_row(row: &rusqlite::Row) -> Result<PomodoroDTO, rusqlite::Error> {
        let start_time: String = row.get(1)?;
        let end_time: Option<String> = row.get(2)?;
        Ok(
            PomodoroDTO {
                id: row.get(0)?,
                start_time: DateTime::parse_from_rfc3339(&start_time).unwrap().to_utc(), // todo: convert str to datetime<utc>?
                end_time: if end_time.is_some() { Some(DateTime::parse_from_rfc3339(&end_time.unwrap()).unwrap().to_utc()) } else { None },
                duration: row.get(3)?,
                note: row.get(4)?,
            }
        )
    }
}

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

    pub fn find_all(&self) {
        let mut stmt = self
            .database_connection
            .prepare("SELECT id, start_time, end_time, duration, note FROM pomodoro")
            .expect("Failed to create database query");
    
        let pomo_iter = stmt
            .query_map([], |row| {   
                PomodoroDTO::from_row(row)
            })
            .unwrap();
    
        for pomo in pomo_iter {
            println!("{:?}", pomo.unwrap());
        }
    }
}
