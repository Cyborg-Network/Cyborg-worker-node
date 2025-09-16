use crate::substrate_interface::api::runtime_types::cyborg_primitives::task::TaskInfo;
use crate::{error::Result, substrate_interface};
use subxt::utils::AccountId32;
use subxt::{OnlineClient, PolkadotConfig};

// Struct that contains the data that the worker needs to execute a task
#[allow(dead_code)]
pub struct CyborgTask {
    pub id: u64,
    pub owner: AccountId32,
    pub cid: String,
}

/// Query the current task id of a miner from the parachain
// No good for producion, but miners need a different ID in order to efficiently query them due to subxt bug when querying tuples
pub async fn get_currently_assigned_task_id(api: &OnlineClient<PolkadotConfig>, miner_id: &(AccountId32, u64)) -> Result<u64> {
    let miner_iter_address = substrate_interface::api::storage()
        .edge_connect()
        .executable_workers_iter();

    let mut miner_iter_query = api
        .storage()
        .at_latest()
        .await?
        .iter(miner_iter_address)
        .await?;

    while let Some(Ok(fetched_miner)) = miner_iter_query.next().await {
       if fetched_miner.value.owner == miner_id.0 && fetched_miner.value.id == miner_id.1 {
            if let Some(task_id) = fetched_miner.value.current_task {
                return Ok(task_id);
            }
       }
    }

    Err("Miner not found".into())
}

pub async fn get_task(api: &OnlineClient<PolkadotConfig>, task_id: u64) -> Result<TaskInfo<AccountId32, u32>> {
    let task_address = substrate_interface::api::storage()
        .task_management()
        .tasks(task_id);

    let task_query: Option<TaskInfo<AccountId32, u32>> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&task_address)
        .await?;

    if let Some(task) = task_query {
        Ok(task)
    } else {
        Err("Task not found".into())
    }
}

pub async fn get_miner_id_assigned_to_task(api: &OnlineClient<PolkadotConfig>, task_id: u64) -> Result<(AccountId32, u64)> {
    let task_address = substrate_interface::api::storage()
        .task_management()
        .task_allocations(task_id);

    let task_query: Option<(AccountId32, u64)> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&task_address)
        .await?;

    if let Some(task) = task_query {
        Ok(task)
    } else {
        Err("Task not found".into())
    }
}

pub async fn get_miner_by_domain(api: &OnlineClient<PolkadotConfig>, local_domain: &String) -> Result<(AccountId32, u64)> {
    let miner_address = substrate_interface::api::storage()
        .edge_connect()
        .executable_workers_iter();

    let mut miner_query = api
        .storage()
        .at_latest()
        .await?
        .iter(miner_address)
        .await?;

    while let Some(Ok(miner)) = miner_query.next().await {
        let queried_domain = String::from_utf8(miner.value.api.domain.0)?;
        if *local_domain == queried_domain {
            return Ok((miner.value.owner, miner.value.id));
        }
    }

    Err("Miner not found".into())
}