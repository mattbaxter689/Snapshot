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
    /// Run the full data checks and save the information
    Ingest {
        /// The config file for the respective data being tested
        #[arg(long, short, default_value = "features.toml")]
        config: PathBuf,

        /// The file to be tested. Either parquet of CSV
        #[arg(long, short, default_value = "data.parqut")]
        file: PathBuf,
    },

    /// Infer an auto TOML schema based on data supplied
    Infer {
        /// The path to the config file for the respective data being tested
        #[arg(long, short, default_value = "features.toml")]
        config: PathBuf,
    },

    /// Create the plan and verify with the user to ensure correctness
    Plan {},
}
