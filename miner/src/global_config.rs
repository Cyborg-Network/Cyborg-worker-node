use once_cell::sync::Lazy;
use once_cell::sync::OnceCell;
use serde::Deserialize;
use std::fs;
use std::{env, path::PathBuf};
use subxt::utils::AccountId32;
use subxt::OnlineClient;
use subxt::PolkadotConfig;

use crate::error::{Error, Result};
use crate::utils::tx_queue::TransactionQueue;
use crate::utils::tx_queue::TRANSACTION_QUEUE;

#[derive(Debug)]
pub struct Paths {
    pub log_path: PathBuf,
    pub task_file_name: String,
    pub task_dir_path: String,
    pub task_owner_path: String,
    pub identity_path: String,
    pub update_path: PathBuf,
}

#[derive(Deserialize, Debug)]
struct MinerIdentity {
    owner: AccountId32,
    id: u32,
}

// We're setting a few global variables here for easy access throughout. If editing, make sure to add appropriate Lazy::force to `run_global_config` - THIS IS NOT COMPILE-TIME ENFORCED
// Paths required throughout
pub static PATHS: Lazy<Paths> = Lazy::new(|| Paths {
    log_path: env::var("LOG_FILE_PATH")
        .expect("LOG_PATH must be set")
        .into(),
    task_file_name: env::var("TASK_FILE_NAME").expect("TASK_FILE_NAME must be set"),
    task_dir_path: env::var("TASK_DIR_PATH").expect("TASK_DIR_PATH must be set"),
    task_owner_path: env::var("TASK_OWNER_FILE_PATH").expect("TASK_OWNER_PATH must be set"),
    identity_path: env::var("IDENTITY_FILE_PATH").expect("IDENTITY_PATH must be set"),
    update_path: env::var("UPDATE_STAGER_PATH")
        .expect("UPDATE_STAGER_PATH must be set")
        .into(),
});

// The tailscale network that the miner is currently operating under
pub static TAILSCALE_NET: Lazy<String> =
    Lazy::new(|| env::var("TAILSCALE_NET").expect("TAILSCALE_NET must be set"));

// The port reserved for the FlashInfer service
pub static FLASH_INFER_PORT: Lazy<u16> = Lazy::new(|| {
    env::var("FLASH_INFER_PORT")
        .expect("FLASH_INFER_PORT must be set")
        .parse()
        .expect("Failed to parse FLASH_INFER_PORT")
});

/*
// The gateway for CESS network
pub static CESS_GATEWAY: Lazy<Arc<RwLock<String>>> = Lazy::new(||
    Arc::new(RwLock::new(String::from("https://deoss-sgp.cess.network")))
);
*/

/// The metadata for the current task in case the miner shuts down unexpectedly and has to restart a running task
pub static CURRENT_TASK_PATH: Lazy<PathBuf> = Lazy::new(|| {
    env::var("CURRENT_TASK_PATH")
        .expect("CURRENT_TASK_PATH must be set")
        .into()
});

/// The client used to connect to the parachain
pub static PARACHAIN_CLIENT: OnceCell<OnlineClient<PolkadotConfig>> = OnceCell::new();

/// Prefix used for container names so that all containers with this prefix can be managed at once by the miner
pub static CONTAINER_PREFIX: &str = "cy-miner-task-container-";

/// Runs the configuration for the miner, everything in this function will fail fast to ensure correct setup when starting the miner
///
/// # Arguments
/// * `parachain_url` - A string representing the URL of the parachain node to connect to.
pub async fn run_global_config(parachain_url: &str) -> Result<()> {
    dotenv::dotenv().ok();

    let parachain_url = if let Ok(parachain_url_env) = env::var("PARACHAIN_URL") {
        parachain_url_env
    } else {
        parachain_url.to_string()
    };

    Lazy::force(&PATHS);
    Lazy::force(&TAILSCALE_NET);
    Lazy::force(&FLASH_INFER_PORT);
    //Lazy::force(&CESS_GATEWAY);
    Lazy::force(&CURRENT_TASK_PATH);

    // Set the transaction queue
    if let Err(_) = TRANSACTION_QUEUE.set(TransactionQueue::new()) {
        panic!("Failed to set transaction queue.");
    }

    // Create a parachain client
    let client = OnlineClient::<PolkadotConfig>::from_url(parachain_url)
        .await
        .expect("Failed to connect to parachain node");

    // Set the client
    PARACHAIN_CLIENT
        .set(client)
        .expect("Client is already initialized!");

    Ok(())
}

pub fn get_parachain_client() -> Result<&'static OnlineClient<PolkadotConfig>> {
    PARACHAIN_CLIENT
        .get()
        .ok_or(Error::parachain_client_not_intitialized())
}

pub fn get_tx_queue() -> Result<&'static TransactionQueue> {
    TRANSACTION_QUEUE.get().ok_or(Error::Custom(
        "Transaction queue not initialized".to_string(),
    ))
}

/*
pub async fn get_cess_gateway() -> String {
    CESS_GATEWAY.read().await.clone()
}

pub async fn set_cess_gateway(url: &str) {
    let mut write_guard = CESS_GATEWAY.write().await;

    *write_guard = url.to_string();
}
*/

pub fn update_config_file(path: &str, content: &str) -> Result<()> {
    let path = PathBuf::from(path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(&path, content)?;

    Ok(())
}
