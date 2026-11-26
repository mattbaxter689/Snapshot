use clap::Parser;
use snapshot::{
    cli::{Cli, Command},
    spec::feature_config::FeatureSetConfig,
};

fn main() {
    match Cli::parse().command {
        Command::Ingest { config, file } => {
            match FeatureSetConfig::load(&config) {
                Ok(cfg) => println!("config loaded: {} features to check", cfg.features.len()),
                Err(e) => {
                    eprintln!("Error: {e:?}"); // {:?} prints the full anyhow context chain
                    std::process::exit(1);
                }
            }
        }
        Command::Infer { config } => println!("config loc {}", config.display()),
    }
}
