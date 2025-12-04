use serde::{Deserialize, Serialize};
use subxt::utils::AccountId32;

pub mod substrate_interface;

pub type MinerIdVec = Vec<u8>;

pub type TaskType = substrate_interface::api::runtime_types::cyborg_primitives::task::TaskKind<u32>;

#[derive(Deserialize, Serialize, Debug)]
pub struct MinerIdentity {
    pub miner_owner: AccountId32,
    pub miner_id: MinerIdVec,
    pub miner_type: substrate_interface::api::runtime_types::cyborg_primitives::miner::MinerType,
}

#[derive(Debug, Clone)]
pub struct CurrentTask {
    pub id: u64,
    pub task_type: TaskType,
    pub container_name: String,
    pub task_owner: AccountId32,
    pub status_sender: tokio::sync::watch::Sender<TaskPreparationStatus>,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskPreparationStatus {
    Preparing,
    Ready,
    Failed(String),
}
