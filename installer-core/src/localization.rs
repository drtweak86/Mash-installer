use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const DEFAULT_STRINGS: &str = include_str!("../../resources/strings/default.toml");

#[derive(Clone, Debug, Deserialize)]
pub struct PhaseStrings {
    pub label: String,
    pub description: String,
}

impl PhaseStrings {
    fn from_parts(label: &str, description: &str) -> Self {
        Self {
            label: label.to_string(),
            description: description.to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct LocalizationFile {
    phases: HashMap<String, PhaseStrings>,
    #[serde(default)]
    general: HashMap<String, String>,
}

#[derive(Clone, Debug)]
pub struct Localization {
    phases: HashMap<String, PhaseStrings>,
    general: HashMap<String, String>,
}

impl Localization {
    pub fn load() -> Result<Self> {
        if let Ok(path) = env::var("MASH_STRINGS_PATH") {
            let path = PathBuf::from(path);
            Self::load_from_path(&path)
        } else {
            Self::load_default()
        }
    }

    pub fn load_default() -> Result<Self> {
        Self::from_str(DEFAULT_STRINGS)
    }

    pub fn load_from_path(path: &Path) -> Result<Self> {
        let text = fs::read_to_string(path)
            .with_context(|| format!("reading localization strings from {}", path.display()))?;
        Self::from_str(&text)
    }

    fn from_str(input: &str) -> Result<Self> {
        let file: LocalizationFile =
            toml::from_str(input).context("parsing localization strings from TOML")?;
        Ok(Self {
            phases: file.phases,
            general: file.general,
        })
    }

    pub fn phase_or_default(
        &self,
        key: &str,
        default_label: &str,
        default_description: &str,
    ) -> PhaseStrings {
        self.phases
            .get(key)
            .cloned()
            .unwrap_or_else(|| PhaseStrings::from_parts(default_label, default_description))
    }

    pub fn general(&self, key: &str) -> Option<&str> {
        self.general.get(key).map(|s| s.as_str())
    }
}
