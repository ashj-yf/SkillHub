use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub api_url: String,
    pub skills_dir: PathBuf,
    pub token: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        let skills_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("skillhub")
            .join("skills");

        Self {
            api_url: "http://localhost:3000/api".to_string(),
            skills_dir,
            token: None,
        }
    }
}

impl Config {
    pub fn config_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("skillhub")
            .join("config.json")
    }

    pub fn load() -> Result<Self> {
        let path = Self::config_path();

        if path.exists() {
            let content = std::fs::read_to_string(&path)
                .context(format!("Failed to read config file: {}", path.display()))?;

            serde_json::from_str(&content)
                .context("Failed to parse config file. The file may be corrupted. Try running 'skillhub init' to reset.")
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::config_path();

        let parent = path.parent()
            .ok_or_else(|| anyhow::anyhow!("Cannot determine config directory parent"))?;

        std::fs::create_dir_all(parent)
            .context("Failed to create config directory")?;

        let content = serde_json::to_string_pretty(self)
            .context("Failed to serialize config")?;

        std::fs::write(&path, content)
            .context(format!("Failed to write config file: {}", path.display()))?;

        Ok(())
    }
}