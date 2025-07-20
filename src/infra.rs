use std::{env, fs};

use rusqlite;

use super::pomo;
use crate::AppSettings;

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

pub fn create_database_connection(settings: &AppSettings) -> std::io::Result<rusqlite::Connection> {
    let home_dir = env::home_dir().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Home directory was not properly recognized!",
        )
    })?;
    let config_dir = home_dir.join(".config").join(&settings.application_name);
    let database_path = config_dir.join(&settings.sqlite_database_name);
    fs::create_dir_all(config_dir)?;
    Ok(
        rusqlite::Connection::open(database_path).unwrap(), // how to create custom error based on the lib error?
    )
}
