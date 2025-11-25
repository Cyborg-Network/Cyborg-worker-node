use std::{path::PathBuf, sync::Arc};

use anyhow::Result;
use tokio::sync::RwLock;

mod api;
mod client;
mod config;
mod formats;
mod crypto;
mod auth;
mod error_handling;
use types::{CurrentTask as RawCurrentTask};

pub type TaskOwner = String;
pub type CurrentTask = Arc<RwLock<Option<RawCurrentTask>>>;

pub struct AgentConfig {
    pub current_task: CurrentTask,
    pub log_file_path: &'static PathBuf,
    pub container_prefix: &'static str,
}

pub async fn run_agent(config: Arc<AgentConfig>) ->Result<()> {
    // initialize logger
    //let old_config_str = include_str!("log.yml");

    //let old_config = serde_yaml::from_str(old_config_str)?;

    //log4rs::init_raw_config(old_config)?;

    println!("Starting Cyborg Agent...");

    client::run_client(config).await?;

    Ok(())
}
