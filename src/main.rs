mod app;
mod infra;
mod pomo;

use clap::{Parser, Subcommand};

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


fn main() {
    let container = infra::container::Container::new(infra::container::AppSettings::new()).unwrap(); // todo: ? instead of unwrap?

    let cli = Cli::parse();

    match &cli.command {
        Commands::Start { duration: _, alert } => app::start_pomodoro(&container),
        Commands::Done {} => println!("Done"),
        Commands::Log {} => println!("Log past pomodoros"),
    }
}
