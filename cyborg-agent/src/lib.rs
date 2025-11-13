use std::{path::PathBuf, sync::Arc};

use anyhow::Result;
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

mod api;
mod client;
mod config;
mod formats;
mod crypto;
mod auth;
mod error_handling;
use types::{CurrentTask as RawCurrentTask};
//#[cfg(test)]
//mod unit_tests;

lazy_static::lazy_static! {
    /// uses [home::home_dir] to determine $HOME or its equivalent
    pub static ref CONFIG_PATH: PathBuf = home::home_dir()
        .expect("Failed to get home directory")
        .join(".config/cyborg/config.toml");

    // pub static ref RELEASE_SERVER_URL: String = "https://localhost:9000/releases".to_string()
    //     + &format!(
    //         "v{}.{}.{}/",
    //         pkg_version_major!(),
    //         pkg_version_minor!(),
    //         pkg_version_patch!()
    //     )
    //     + "scripts/";
}

pub type TaskOwner = String;
pub type CurrentTask = Arc<RwLock<Option<RawCurrentTask>>>;

pub struct Paths {
    pub task_owner: PathBuf,
    pub miner_config: PathBuf,
    pub logs: PathBuf,
}

pub static TASK_CONTAINER_PREFIX: Lazy<String> = Lazy::new(|| {
    std::env::var("TASK_CONTAINER_PREFIX").expect("TASK_CONTAINER_PREFIX not set")
});

pub static PATHS: Lazy<Paths> = Lazy::new(|| {
    Paths{
        task_owner: std::env::var("TASK_OWNER_FILE_PATH").expect("TASK_OWNER_FILE_PATH not set").into(),
        miner_config: std::env::var("IDENTITY_FILE_PATH").expect("IDENTITY_FILE_PATH not set").into(),
        logs: std::env::var("LOG_FILE_PATH").expect("LOG_FILE_PATH not set").into(),
    }
});

pub struct AgentConfig<'a> {
    pub current_task: CurrentTask,
    pub log_file_path: &'a PathBuf,
}

pub async fn run_agent<'a>(config: AgentConfig<'a>) ->Result<()> {
    Lazy::force(&TASK_CONTAINER_PREFIX);
    Lazy::force(&PATHS);

    // initialize logger
    let old_config_str = include_str!("log.yml");

    let old_config = serde_yaml::from_str(old_config_str)?;

    log4rs::init_raw_config(old_config)?;

    println!("Running");

    client::run_client(config).await?;

    Ok(())
}
