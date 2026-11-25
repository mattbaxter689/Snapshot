use std::{collections::HashSet, fs, path::Path};

use anyhow::{Context, Result, bail};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FeatureSetConfig {
    pub meta: FeatureSpecMeta,
    pub features: Vec<FeatureRule>,
    pub format: Option<Format>,
    pub feature_path: String,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct FeatureSpecMeta {
    pub name: String,
    pub entity_key: String,
    pub event_timestamp: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FeatureRule {
    pub name: String,
    pub min_bound: Option<f32>,
    pub max_bound: Option<f32>,
    pub max_null_ratio: f32,
    pub sql_expr: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Format {
    Csv,
    Parquet,
}

impl FeatureSpecMeta {
    pub fn validate(&self) -> Option<Vec<&str>> {
        let missing: Vec<&str> = [
            ("name", &self.name),
            ("entity_key", &self.entity_key),
            ("event_timestamp", &self.event_timestamp),
        ]
        .into_iter()
        .filter_map(|(field, val)| {
            if val.trim().is_empty() {
                Some(field)
            } else {
                None
            }
        })
        .collect();

        if missing.is_empty() {
            None
        } else {
            Some(missing)
        }
    }
}

impl FeatureSetConfig {
    pub fn format(&self) -> Format {
        self.format.unwrap_or_else(|| {
            if self.feature_path.to_lowercase().ends_with(".csv") {
                Format::Csv
            } else {
                Format::Parquet
            }
        })
    }

    pub fn load(path: &Path) -> Result<Self> {
        let text = fs::read_to_string(path)
            .with_context(|| format!("Reading config {}", path.display()))?;
        Self::from_toml(&text).with_context(|| format!("in config {}", path.display()))
    }

    pub fn from_toml(text: &str) -> Result<Self> {
        let cfg: FeatureSetConfig =
            toml::from_str(text).context("Invalid TOML or unknown field")?;
        // Validate the config before we do anything else
        // This is for testing
        Ok(cfg)
    }

    fn validate(&self) -> Result<()> {
        // ensure that feature set is there
        if let Some(missing_meta) = self.meta.validate() {
            bail!(
                "Missing the following meta information in TOML: {}",
                missing_meta.join(",")
            )
        }

        //check to see that features are provided
        if self.features.is_empty() {
            bail!("No [[feature]] entries declared");
        }

        let mut seen = HashSet::new();
        // for feature in self.features {}
    }
}
