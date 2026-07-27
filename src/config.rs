use std::{fs, path::Path};

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub version: u32,
    #[serde(default)]
    pub trust: TrustConfig,
    pub checks: Vec<CheckConfig>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TrustConfig {
    /// A local PEM bundle. No trust list is downloaded at runtime.
    pub anchors_path: Option<String>,
    /// A human-readable identifier or immutable source URL recorded in results.
    pub id: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CheckConfig {
    pub id: String,
    pub checkpoints: Vec<CheckpointConfig>,
    #[serde(default)]
    pub policy: PolicyConfig,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CheckpointConfig {
    pub name: String,
    pub path: Option<String>,
    pub url: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PolicyConfig {
    #[serde(default)]
    pub require_credential: bool,
    #[serde(default)]
    pub require_valid_binding: bool,
    #[serde(default)]
    pub require_trusted: bool,
    #[serde(default)]
    pub continuity: ContinuityPolicy,
    #[serde(default = "default_inconclusive")]
    pub on_inconclusive: InconclusivePolicy,
}

impl Default for PolicyConfig {
    fn default() -> Self {
        Self {
            require_credential: false,
            require_valid_binding: false,
            require_trusted: false,
            continuity: ContinuityPolicy::Any,
            on_inconclusive: InconclusivePolicy::Warn,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ContinuityPolicy {
    #[default]
    Any,
    Exact,
    ExactOrDeclaredDerivative,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum InconclusivePolicy {
    Pass,
    #[default]
    Warn,
    Fail,
}

fn default_inconclusive() -> InconclusivePolicy {
    InconclusivePolicy::Warn
}

impl Config {
    pub fn from_path(path: &Path) -> Result<Self> {
        let raw = fs::read_to_string(path)
            .with_context(|| format!("failed to read config {}", path.display()))?;
        let config: Self = serde_yaml::from_str(&raw)
            .with_context(|| format!("invalid YAML config {}", path.display()))?;
        config.validate(path)?;
        Ok(config)
    }

    fn validate(&self, config_path: &Path) -> Result<()> {
        if self.version != 1 {
            bail!("unsupported config version {}; expected 1", self.version);
        }
        if self.checks.is_empty() {
            bail!("config must contain at least one check");
        }

        let base = config_path.parent().unwrap_or_else(|| Path::new("."));
        if let Some(path) = &self.trust.anchors_path {
            let path = base.join(path);
            if !path.is_file() {
                bail!("trust anchors file does not exist: {}", path.display());
            }
        }

        let mut ids = std::collections::HashSet::new();
        for check in &self.checks {
            if check.id.trim().is_empty() {
                bail!("check id must not be empty");
            }
            if !ids.insert(check.id.as_str()) {
                bail!("duplicate check id: {}", check.id);
            }
            if check.checkpoints.is_empty() {
                bail!("check {} must contain at least one checkpoint", check.id);
            }
            let mut names = std::collections::HashSet::new();
            for checkpoint in &check.checkpoints {
                if checkpoint.name.trim().is_empty() {
                    bail!("checkpoint name must not be empty in check {}", check.id);
                }
                if !names.insert(checkpoint.name.as_str()) {
                    bail!(
                        "duplicate checkpoint name {} in check {}",
                        checkpoint.name,
                        check.id
                    );
                }
                match (&checkpoint.path, &checkpoint.url) {
                    (Some(_), None) | (None, Some(_)) => {}
                    _ => bail!(
                        "checkpoint {} in check {} must set exactly one of path or url",
                        checkpoint.name,
                        check.id
                    ),
                }
            }
        }
        Ok(())
    }
}
