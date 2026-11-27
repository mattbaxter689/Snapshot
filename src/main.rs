use clap::Parser;
use snapshot::{
    cli::{Cli, Command},
    spec::{feature_config::FeatureSetConfig, table_spec::TableSpec},
};
use std::path::{Path, PathBuf};

fn main() {
    match Cli::parse().command {
        Command::Ingest { config, file } => {
            if let Err(e) = run_ingest(&config, file) {
                eprintln!("Error: {e:?}"); // {:?} prints the full anyhow context chain
                std::process::exit(1);
            };
        }
        Command::Infer { config } => println!("config loc {}", config.display()),
        Command::Plan {} => todo!(),
    }
}

fn run_ingest(config_path: &Path, file: PathBuf) -> anyhow::Result<()> {
    let cfg = FeatureSetConfig::load(config_path)?;
    println!("config loaded: {} features to check", cfg.features.len());

    let table_spec = TableSpec::new(&cfg, file)?;
    println!("Using file path: {}", table_spec.file_name);
    println!("Required columns: {:?}", table_spec.required_columns);

    // next: register the table with DataFusion using table_spec.file_path / table_spec.format
    Ok(())
}
