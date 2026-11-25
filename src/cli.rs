use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "snapshot",
    version,
    about = "Data transformation validation CLI audit"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Ingest {
        #[arg(long, short, default_value = "features.toml")]
        config: PathBuf,

        #[arg(long, short, default_value = "data.parqut")]
        file: PathBuf,
        // This will be used later
        // #[arg(long, short)]
        // file: PathBuf,
    },
    Infer {
        #[arg(long, short, default_value = "features.toml")]
        config: PathBuf,
    },
}
