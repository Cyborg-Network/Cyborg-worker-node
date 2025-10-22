use crate::substrate_interface::api::edge_connect::calls::types::remove_miner::MinerId;
use crate::substrate_interface::api::runtime_types::bounded_collections::bounded_vec::BoundedVec;
use crate::substrate_interface::api::runtime_types::cyborg_primitives::miner::MinerType;
use crate::substrate_interface::api::runtime_types::cyborg_primitives::task::TaskInfo;
use crate::types::MinerIdentity;
use crate::{error::Result, substrate_interface};
use std::sync::Arc;
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
pub async fn get_currently_assigned_task_id(
    api: &OnlineClient<PolkadotConfig>,
    miner_id: &MinerId,
    miner_type: Arc<MinerType>,
) -> Result<u64> {
    let miner_iter_address = match miner_type.as_ref() {
        MinerType::Edge => substrate_interface::api::storage()
            .edge_connect()
            .edge_miners_iter(),
        MinerType::Cloud => substrate_interface::api::storage()
            .edge_connect()
            .cloud_miners_iter(),
    };

    let mut miner_iter_query = api
        .storage()
        .at_latest()
        .await?
        .iter(miner_iter_address)
        .await?;

    while let Some(Ok(fetched_miner)) = miner_iter_query.next().await {
        if fetched_miner.value.id == miner_id.clone() {
            if let Some(task_id) = fetched_miner.value.current_task {
                return Ok(task_id);
            } else {
                return Err("Miner has no task assigned".into());
            }
        }
    }

    Err("Miner not found".into())
}

pub async fn get_task(
    api: &OnlineClient<PolkadotConfig>,
    task_id: u64,
) -> Result<TaskInfo<AccountId32, u32>> {
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

pub async fn get_miner_id_assigned_to_task(
    api: &OnlineClient<PolkadotConfig>,
    task_id: u64,
) -> Result<BoundedVec<u8>> {
    let task_address = substrate_interface::api::storage()
        .task_management()
        .task_allocations(task_id);

    let task_query = api
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

/// TODO - once attestation is in place, this needs to be via ID, not domain
pub async fn get_miner_by_domain(
    api: &OnlineClient<PolkadotConfig>,
    domain: &String,
    miner_type: Arc<MinerType>,
) -> Result<MinerIdentity> {
    let miner_address = match miner_type.as_ref() {
        MinerType::Edge => substrate_interface::api::storage()
            .edge_connect()
            .edge_miners_iter(),
        MinerType::Cloud => substrate_interface::api::storage()
            .edge_connect()
            .cloud_miners_iter(),
    };

    let mut miner_query = api.storage().at_latest().await?.iter(miner_address).await?;

    while let Some(Ok(miner)) = miner_query.next().await {
        let queried_domain = String::from_utf8(miner.value.api.domain.0)?;
        if *domain == queried_domain {
            return Ok(MinerIdentity {
                miner_owner: miner.value.owner.clone(),
                miner_id: miner.value.id.clone(),
                miner_type: miner_type.as_ref().clone(),
            });
        }
    }

    Err("Miner not found".into())
}



pub async fn get_miner_by_id(
    api: &OnlineClient<PolkadotConfig>,
    miner_id: String,
) -> Result<MinerIdentity> {
    // Determine miner type and convert ID
    let storage = api.storage().at_latest().await?;
    let miner_id_bytes = miner_id.as_bytes().to_vec();

    let (miner_type, miner_iter_addr) = if miner_id.starts_with("ED-") {
        (
            MinerType::Edge,
            substrate_interface::api::storage().edge_connect().edge_miners_iter(),
        )
    } else if miner_id.starts_with("CL-") {
        (
            MinerType::Cloud,
           substrate_interface::api::storage().edge_connect().cloud_miners_iter(),
        )
    } else {
        return Err("Invalid miner ID prefix — must start with ED- or CL-".into());
    };
    let mut miner_iter = storage.iter(miner_iter_addr).await?;
    let mut found_miner = None;

    while let Some(Ok(kv)) = miner_iter.next().await {
        let value = kv.value;
        if value.id.0 == miner_id_bytes {
            found_miner = Some(value);
            break;
        }
    }

let miner_info = found_miner.ok_or_else(|| "Miner not found for given ID")?;

    Ok(MinerIdentity {
        miner_owner: miner_info.owner,
        miner_id: miner_info.id.clone(),
        miner_type,
    })
}
