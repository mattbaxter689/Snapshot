use clap::Parser;
use snapshot::cli::{Cli, Command};

fn main() {
    match Cli::parse().command {
        Command::Ingest { config } => println!("config loc {}", config.display()),
        Command::Infer { config } => println!("config loc {}", config.display()),
    }
}
