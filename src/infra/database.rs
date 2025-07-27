use std::{env, fs};

pub fn create_database_connection(settings: &super::container::AppSettings) -> crate::error::GenericResult<rusqlite::Connection> {
    let home_dir = env::home_dir().unwrap();
    let config_dir = home_dir.join(".config").join(&settings.application_name);
    let database_path = config_dir.join(&settings.sqlite_database_name);
    fs::create_dir_all(config_dir)?;
    Ok(
        rusqlite::Connection::open(database_path)?
    )
}

pub fn apply_database_migrations(connection: &rusqlite::Connection) {
    // todo: create/reuse migration management system
    connection.execute(
        "CREATE TABLE IF NOT EXISTS pomodoro (
            id CHARACTER(36) PRIMARY KEY,
            start_time DATETIME NOT NULL,
            end_time DATETIME,
            duration INT NOT NULL,
            note TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        (),
    ).unwrap();
}
