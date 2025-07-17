mod app;
mod infra;
mod pomo;

use std::{env, fs, path::Path};

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

struct Container {
    database_connection: Option<Connection>,
}

impl Container {
    fn new() -> Container {
        Container {
            database_connection: None,
        }
    }

    /// Initialize database connection to sqlite at the default location.
    fn with_database_connection(mut self) -> Self {
        let home_dir = env::home_dir().unwrap();
        let database_path = home_dir.join(Path::new(".config/pomo-cli-rust/database.db")); // todo: might make sense to make it configurable
        fs::create_dir_all(database_path.parent().unwrap()).unwrap();
        self.database_connection = Some(infra::create_database_connection(
            database_path.to_str().unwrap(),
        ));
        println!("{}", database_path.display());
        self
    }

    fn with_database_migrations(self) -> Self {
        self
    }
}

fn main() {
    let container = Container::new()
        .with_database_connection()
        .with_database_migrations();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Start { duration: _, alert } => app::start_pomodoro(),
        Commands::Done {} => println!("Done"),
        Commands::Log {} => println!("Log past pomodoros"),
    }
}
