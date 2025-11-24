use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, str::FromStr};

use uuid::Uuid;

mod migrations;

/// the specific current version of the *config*. this may not necessarily match the crate version.
/// this version number should be changed after any change to the structure of the configuration storage.
/// defining migrations from previous config versions is also required for backwards compatibility
const CURRENT_VERSION: &str = "v0.1.0";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Configuration {
    pub version: String,
    /// basic configuration
    pub base: BaseConfig,
    /// special configuration
    pub runtime: RuntimeConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
/// basic configuration fields
pub struct BaseConfig {
    /// to accept connections from the frontend to connect to
    pub websocket_server_url: String,
    /// to accept connections from the oracle to connect to
    pub oracle_url: String,
    /// used for identification + authentication
    pub csc_uuid: String,
    /// used for authentication
    pub user_token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RuntimeConfig {
    /// the timeout in milliseconds
    pub timeout: u64,
}

impl Configuration {
    fn new(user_token: String) -> Self {
        Configuration {
            version: CURRENT_VERSION.to_string(),
            base: BaseConfig {
                websocket_server_url: "http://localhost:3000".to_string(),
                oracle_url: "oracle_url".to_string(),
                // create a new rando csc uuid
                csc_uuid: Uuid::new_v4().to_string(),
                user_token,
            },
            runtime: RuntimeConfig { timeout: 10_000 },
        }
    }
}

/// creating a new default configuration with options
pub fn create_config(config_path: &PathBuf, force: bool, token: String) -> Result<()> {
    if config_path.exists() && !force {
        bail!("Configuration file already exists. Use --force to overwrite.");
    }
    let config = Configuration::new(token);
    let config_str = toml::to_string_pretty(&config).context("Failed to serialize config")?;
    let prefix = config_path.parent().unwrap();
    // create every needed directory
    std::fs::create_dir_all(prefix).context("Failed to create config directory")?;
    // write the config file
    std::fs::write(config_path, config_str).context("Failed to write config")?;
    Ok(())
}

/// tries to load a configuration. may require a migration to up-to-date config version.
pub fn load_config(config_path: &PathBuf) -> Result<Configuration> {
    let config_str = std::fs::read_to_string(config_path).context("Failed to read config")?;
    let value = toml::Value::from_str(&config_str).context("Failed to parse config as TOML")?;
    let config: Configuration = match value.get("version").and_then(|a| a.as_str()) {
        Some(CURRENT_VERSION) => {
            toml::from_str(&config_str).context("Failed to deserialize config")?
        }
        Some(v) => {
            bail!("Unsupported configuration version: {}", v)
        }
        None => {
            println!(
                "Failed to read config. To migrate to version {}, place the following into your config file: \n{}",
                CURRENT_VERSION,
                toml::to_string_pretty(&migrations::none::create_config(value).context("Failed to deserialize config")?).unwrap()
            );
            bail!(
                "Config version: version None must be upgraded to {}",
                CURRENT_VERSION
            )
        }
    };
    Ok(config)
}
