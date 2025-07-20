mod app;
mod infra;
mod pomo;

use std::{env, fs, io};

use clap::{Parser, Subcommand};
use rusqlite::Connection;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start new pomodoro session
    Start {
        /// Duration in minutes, for example `--duration 30` to begin 30 minute pomodoro.
        #[arg(short, long)]
        duration: Option<u32>,

        #[arg(short, long)]
        alert: bool,
    },
    /// Finish active pomodoro.
    Done {},
    /// Show past pomodoros.
    Log {},
}

struct AppSettings {
    application_name: String,
    sqlite_database_name: String,
}

impl AppSettings {
    fn new() -> AppSettings {
        AppSettings {
            application_name: "pomo-cli-rust".to_string(),
            sqlite_database_name: "database.db".to_string(),
        }
    }
}

struct Container {
    settings: AppSettings,
    database_connection: Connection,
}

impl Container {
    fn new(settings: AppSettings) -> std::io::Result<Container> {
        let connection = infra::create_database_connection(&settings)?;
        Ok(Container {
            settings: settings,
            database_connection: connection,
        })
    }
}

fn main() {
    let container = Container::new(AppSettings::new()).unwrap(); // todo: ? instead of unwrap?

    let cli = Cli::parse();

    match &cli.command {
        Commands::Start { duration: _, alert } => app::start_pomodoro(&container),
        Commands::Done {} => println!("Done"),
        Commands::Log {} => println!("Log past pomodoros"),
    }
}
