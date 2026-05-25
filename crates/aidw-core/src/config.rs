//! Configuration management for .aidw.toml

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Config file not found at {0}")]
    NotFound(PathBuf),
    #[error("Failed to read config: {0}")]
    ReadError(#[from] std::io::Error),
    #[error("Failed to parse config: {0}")]
    ParseError(#[from] toml::de::Error),
    #[error("Failed to serialize config: {0}")]
    SerializeError(#[from] toml::ser::Error),
}

/// Main configuration structure stored in .aidw.toml
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub project: ProjectConfig,
    #[serde(default)]
    pub commands: CommandsConfig,
    #[serde(default)]
    pub paths: PathsConfig,
    #[serde(default)]
    pub current_task: Option<TaskConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    pub stack: String,
    #[serde(default = "default_language")]
    pub language: String,
}

fn default_language() -> String {
    "pt-BR".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CommandsConfig {
    #[serde(default)]
    pub lint: Option<String>,
    #[serde(default)]
    pub typecheck: Option<String>,
    #[serde(default)]
    pub test: Option<String>,
    #[serde(default)]
    pub build: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathsConfig {
    #[serde(default = "default_progress_path")]
    pub progress: String,
    #[serde(default = "default_adr_dir")]
    pub adr_dir: String,
    #[serde(default = "default_features_dir")]
    pub features_dir: String,
}

impl Default for PathsConfig {
    fn default() -> Self {
        Self {
            progress: default_progress_path(),
            adr_dir: default_adr_dir(),
            features_dir: default_features_dir(),
        }
    }
}

fn default_progress_path() -> String {
    "docs/progress/PROGRESS.md".to_string()
}

fn default_adr_dir() -> String {
    "docs/adr".to_string()
}

fn default_features_dir() -> String {
    "docs/features".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskConfig {
    pub id: String,
    pub title: String,
    pub phase: String,
    #[serde(default)]
    pub started_at: Option<String>,
}

impl Config {
    /// Default config file name
    pub const FILE_NAME: &'static str = ".aidw.toml";

    /// Create a new config with project defaults
    pub fn new(name: String, stack: String) -> Self {
        Self {
            project: ProjectConfig {
                name,
                stack,
                language: default_language(),
            },
            commands: CommandsConfig::default(),
            paths: PathsConfig::default(),
            current_task: None,
        }
    }

    /// Load config from a directory (looks for .aidw.toml)
    pub fn load(dir: &Path) -> Result<Self, ConfigError> {
        let path = dir.join(Self::FILE_NAME);
        if !path.exists() {
            return Err(ConfigError::NotFound(path));
        }
        let content = std::fs::read_to_string(&path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    /// Save config to a directory
    pub fn save(&self, dir: &Path) -> Result<(), ConfigError> {
        let path = dir.join(Self::FILE_NAME);
        let content = toml::to_string_pretty(self)?;
        std::fs::write(&path, content)?;
        Ok(())
    }

    /// Find config by walking up from the given directory
    pub fn find(start_dir: &Path) -> Option<PathBuf> {
        let mut current = start_dir.to_path_buf();
        loop {
            let config_path = current.join(Self::FILE_NAME);
            if config_path.exists() {
                return Some(current);
            }
            if !current.pop() {
                return None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_config_roundtrip() {
        let dir = tempdir().unwrap();
        let config = Config::new("test-project".to_string(), "nextjs".to_string());
        config.save(dir.path()).unwrap();

        let loaded = Config::load(dir.path()).unwrap();
        assert_eq!(loaded.project.name, "test-project");
        assert_eq!(loaded.project.stack, "nextjs");
    }

    #[test]
    fn test_config_not_found() {
        let dir = tempdir().unwrap();
        let result = Config::load(dir.path());
        assert!(result.is_err());
    }
}
