use clap::Parser;
use snapshot::{
    cli::{Cli, Command},
    config::FeatureSetConfig,
};

fn main() {
    match Cli::parse().command {
        Command::Ingest { config, file } => {
            let config = FeatureSetConfig::load(&config);
            println!("config loaded");
        }
        Command::Infer { config } => println!("config loc {}", config.display()),
    }
}
