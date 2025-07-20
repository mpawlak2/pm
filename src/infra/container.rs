use rusqlite;

pub struct AppSettings {
    pub application_name: String,
    pub sqlite_database_name: String,
}

impl AppSettings {
    pub fn new() -> AppSettings {
        AppSettings {
            application_name: "pomo-cli-rust".to_string(),
            sqlite_database_name: "database.db".to_string(),
        }
    }
}

pub struct Container {
    pub settings: AppSettings,
    pub database_connection: rusqlite::Connection,
}

impl Container {
    pub fn new(settings: AppSettings) -> std::io::Result<Container> {
        let connection = super::create_database_connection(&settings)?;
        Ok(Container {
            settings: settings,
            database_connection: connection,
        })
    }
}
