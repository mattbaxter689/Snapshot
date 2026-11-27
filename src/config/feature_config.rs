use std::vec;
use std::{collections::HashSet, fs, path::Path};

use anyhow::{Context, Result, bail};
use serde::Deserialize;

use crate::config::non_neg::NonNegative;
use crate::sql_verify::verify_sql::{extract_required_columns, parse};

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FeatureSetConfig {
    pub feature_set: FeatureSpecMeta,
    pub features: Vec<FeatureRule>,
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
    pub source_col: Option<String>,
    pub min_bound: Option<f32>,
    pub max_bound: Option<f32>,
    pub max_null_ratio: NonNegative,
    pub sql_expr: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Format {
    Csv,
    Parquet,
}

impl FeatureSpecMeta {
    fn validate(&self) -> Option<Vec<&str>> {
        let missing: Vec<&str> = [
            ("name", &self.name),
            ("entity_key", &self.entity_key),
            ("event_timestamp", &self.event_timestamp),
        ]
        .into_iter()
        .filter_map(|(field, val)| val.trim().is_empty().then_some(field))
        .collect();

        (!missing.is_empty()).then_some(missing)
    }
}

impl FeatureRule {
    fn trimmed(field: &Option<String>) -> Option<&str> {
        field.as_deref().map(str::trim)
    }

    fn validate(&self) -> Option<Vec<String>> {
        let mut errors: Vec<String> = Vec::new();

        // `name` is always required
        let label = match self.name.trim() {
            "" => {
                errors.push("Feature rule is missing a `name` field".to_string());
                "<unnamed>"
            }
            name => name,
        };

        match (
            Self::trimmed(&self.sql_expr),
            Self::trimmed(&self.source_col),
        ) {
            // expression: name is enough, source_col shouldn't be there
            (Some(""), _) => errors.push(format!(
                "Feature `{label}`: `sql_expr` is present but blank"
            )),
            (Some(expr), Some(_)) => {
                errors.push(format!(
                    "Feature `{label}`: `source_col` should not be set when `sql_expr` is used"
                ));
                errors.extend(Self::validate_sql_expr(expr));
            }
            (Some(expr), None) => errors.extend(Self::validate_sql_expr(expr)),

            // no expression: source_col is required
            (None, Some("")) => errors.push(format!(
                "Feature `{label}`: `source_col` is present but blank"
            )),
            (None, Some(_)) => {}
            (None, None) => errors.push(format!(
                "Feature `{label}`: `source_col` is required when `sql_expr` is not set"
            )),
        }
        // Ensure the bounds are correct
        errors.extend(self.ensure_bounds(label));

        (!errors.is_empty()).then_some(errors)
    }

    fn ensure_bounds(&self, rule_label: &str) -> Option<String> {
        match (self.min_bound, self.max_bound) {
            (Some(min), Some(max)) if min > max => Some(format!(
                "Feature `{rule_label}`: min bound ({min}) cannot be greater than max bound ({max})"
            )),
            _ => None,
        }
    }

    fn validate_sql_expr(expr: &str) -> Option<String> {
        parse(expr)
            .err()
            .map(|e| format!("Invalid sql_expr `{expr}`: {e}"))
    }

    pub fn required_columns(&self) -> Result<Vec<String>> {
        match (&self.source_col, &self.sql_expr) {
            (Some(col), None) => Ok(vec![col.trim().to_string()]),
            (None, Some(expr)) => extract_required_columns(expr.trim()),
            _ => bail!(
                "Feature `{}`: needs exactly one of source_col or sql_expr",
                self.name
            ),
        }
    }
}

impl FeatureSetConfig {
    pub fn load(path: &Path) -> Result<Self> {
        let text = fs::read_to_string(path)
            .with_context(|| format!("Reading config {}", path.display()))?;
        Self::from_toml(&text).with_context(|| format!("in config {}", path.display()))
    }

    pub fn from_toml(text: &str) -> Result<Self> {
        let cfg: FeatureSetConfig =
            toml::from_str(text).context("Invalid TOML or unknown field")?;
        // Validate the config
        let results = cfg.validate()?;

        if !results.is_empty() {
            bail!(
                "Errors found in defined features in TOML: {}",
                results.join("\n")
            )
        }
        Ok(cfg)
    }

    fn validate(&self) -> Result<Vec<String>> {
        // ensure that feature set is there
        if let Some(missing_meta) = self.feature_set.validate() {
            bail!(
                "Missing the following meta information in TOML: {}",
                missing_meta.join(",")
            )
        }

        //check to see that features are provided
        if self.features.is_empty() {
            bail!("No [[feature]] entries declared");
        }

        // Validate features
        let mut seen = HashSet::new();
        let mut all_errors: Vec<String> = Vec::new();
        for feature in &self.features {
            if let Some(errors) = feature.validate() {
                all_errors.extend(errors);
            }

            // check if a feature is included twice
            if !seen.insert(feature.name.as_str()) {
                all_errors.push(format!(
                    "Duplicate features found in TOML: `{}`",
                    feature.name,
                ));
            }
        }

        Ok(all_errors)
    }

    pub fn required_columns(&self) -> Result<Vec<String>> {
        let mut cols = vec![
            self.feature_set.entity_key.clone(),
            self.feature_set.event_timestamp.clone(),
        ];
        for rule in &self.features {
            for c in rule.required_columns()? {
                if !cols.contains(&c) {
                    cols.push(c);
                }
            }
        }
        Ok(cols)
    }
}
