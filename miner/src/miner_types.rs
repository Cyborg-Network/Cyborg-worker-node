use serde::{Deserialize, Serialize};
use std::sync::Arc;
use subxt::utils::AccountId32;
use subxt_signer::sr25519::Keypair;
use tokio::sync::RwLock;

use types::{
    substrate_interface::api::{
        edge_connect::calls::types::remove_miner::MinerId,
        runtime_types::cyborg_primitives::{
            miner::MinerType
        }
    },
    CurrentTask
};

use crate::error::Result;

#[derive(Deserialize, Serialize, Debug)]
pub struct MinerIdentity {
    pub miner_owner: AccountId32,
    pub miner_id: MinerId,
    pub miner_type: MinerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskOwner {
    pub task_owner: String,
}

pub struct MinerConfig {
    pub domain: String,
    pub latitude: i32,
    pub longitude: i32,
    pub ram: u64,
    pub storage: u64,
    pub cpu: u16,
}

pub struct ParentRuntime {
    //This is kept as an option, because it might be user dynamic in the future
    pub port: Option<u16>,
}

/// Represents a client for interacting with the Cyborg blockchain.
///
/// This struct is used to interact with the Cyborg blockchain, manage key pairs,
/// and optionally communicate with IPFS or node URIs.
pub struct Miner {
    // For some fields it might not be strictly necessary to be wrapped in an Arc right now, but the expectation is that it will become necessary
    pub miner_type: Arc<MinerType>,
    pub(crate) keypair: Arc<Keypair>,
    pub parent_runtime: Arc<RwLock<ParentRuntime>>,
    pub identity: Arc<MinerIdentity>,
    pub current_task: Arc<RwLock<Option<CurrentTask>>>,
}

impl Miner {
    /// Returns the current task if the miner is active, or an error otherwise
    pub async fn current_task(&self) -> Result<Arc<RwLock<CurrentTask>>> {
        let task_opt = self.current_task.read().await;
        if let Some(task) = task_opt.as_ref() {
            Ok(Arc::new(RwLock::new(task.clone())))
        } else {
            Err("Miner is inactive".into())
        }
    }

    /// Returns true if the miner has an active task
    pub async fn is_active(&self) -> bool {
        self.current_task.read().await.is_some()
    }

    /// Activates the miner with a given task
    pub async fn activate_task(&self, task: CurrentTask) {
        *self.current_task.write().await = Some(task);
    }

    /// Deactivates the miner, returning the task that was active (if any)
    pub async fn deactivate_task(&self) -> Option<CurrentTask> {
        self.current_task.write().await.take()
    }
}
