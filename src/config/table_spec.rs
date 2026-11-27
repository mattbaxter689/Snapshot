use std::path::PathBuf;

use anyhow::{Result, bail};

use crate::config::feature_config::FeatureSetConfig;

#[derive(Debug)]
pub struct TableSpec {
    pub name: String,
    pub required_columns: Vec<String>,
    pub format: Format,
    pub file_path: PathBuf,
    pub file_name: String,
}

#[derive(Debug, Clone, Copy)]
pub enum Format {
    Parquet,
    Csv,
}

impl TableSpec {
    pub fn new(config: &FeatureSetConfig, path: PathBuf) -> Result<Self> {
        let format = match path.extension().and_then(|e| e.to_str()) {
            Some("parquet") => Format::Parquet,
            Some("csv") => Format::Csv,
            _ => bail!("Unsupported file extension. We only accept parquet and csv files"),
        };

        let file_name = path
            .file_stem()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_default();

        Ok(Self {
            name: config.feature_set.name.clone(),
            required_columns: config.required_columns()?,
            format,
            file_path: path,
            file_name,
        })
    }
}
