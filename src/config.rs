use anyhow::Result;
use serde::Deserialize;
use std::fs;
use std::path::Path;

/// Configuration file structure for .suresign.json
#[derive(Deserialize, Debug, Default)]
pub struct Config {
    /// Common Name
    pub cn: Option<String>,

    /// Subject Alternative Names
    pub sans: Option<Vec<String>>,

    /// Validity in days
    pub days: Option<i64>,

    /// Output directory
    pub output: Option<String>,

    /// Output file name (without extension)
    pub name: Option<String>,

    /// PFX password
    pub pfx_password: Option<String>,

    /// Country Name
    pub country: Option<String>,

    /// State or Province
    pub state: Option<String>,

    /// City
    pub city: Option<String>,

    /// Organization
    pub org: Option<String>,

    /// Organizational Unit
    pub org_unit: Option<String>,

    /// Key type (rsa, ecdsa, ed25519)
    pub key_type: Option<String>,
}

impl Config {
    /// Load config from a file path
    pub fn load(path: &Path) -> Result<Option<Self>> {
        if !path.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(path)?;
        let config: Config = serde_json::from_str(&content)?;
        Ok(Some(config))
    }

    /// Try to load config from default locations
    /// 1. .suresign.json in current directory
    /// 2. ~/.suresign.json in home directory
    pub fn load_default() -> Option<Self> {
        // Try current directory first
        if let Ok(Some(config)) = Self::load(Path::new(".suresign.json")) {
            return Some(config);
        }

        // Try home directory
        if let Some(home) = dirs::home_dir() {
            if let Ok(Some(config)) = Self::load(&home.join(".suresign.json")) {
                return Some(config);
            }
        }

        None
    }
}
