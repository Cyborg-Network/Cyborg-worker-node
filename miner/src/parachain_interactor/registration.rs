use crate::error::Result;
use crate::global_config::{self, update_config_file, PATHS};
use crate::self_management::try_apply_update_if_available;
use crate::substrate_interface;
use crate::substrate_interface::api::runtime_types::bounded_collections::bounded_vec::BoundedVec;
use crate::substrate_interface::api::runtime_types::cyborg_primitives::miner::{
    MinerType, OperationalStatus,
};
use crate::traits::ParachainInteractor;
use crate::types::{Miner, MinerIdentity};
use crate::utils::substrate_queries::get_miner_operational_status;
use crate::utils::task_handling::pick_up_task;
use crate::utils::tx_builder::pub_register;
use once_cell::sync::Lazy;
use std::fs;
use std::sync::Arc;
use std::time::{Duration, Instant};
use subxt_signer::sr25519::Keypair;
use tokio::sync::Mutex;

static LAST_UPDATE_CHECK: Lazy<Mutex<Option<Instant>>> = Lazy::new(|| Mutex::new(None));

pub enum RegistrationStatus {
    Registered(MinerIdentity),
    Unknown,
}

async fn confirm_registration() -> Result<RegistrationStatus> {
    //TODO - REGISTRATION VIA ATTESTATION: At the moment we're saving the identity to the config file - this needs to be replaced by invoking the miner-attestor to obtain the pulic key of the miner and verify this way
    let client = global_config::get_parachain_client()?;

    let identity_path = &PATHS.identity_path;
    let identity_file_content = fs::read_to_string(identity_path)?;
    let identity: MinerIdentity = serde_json::from_str(&identity_file_content)?;
    let miner_type = identity.miner_type;
    // let identity = identity.miner_id;
    let miner_id = identity.miner_id.0.clone();

    println!("Confirming miner registration...");

    println!("identity: {:?}", miner_id);

    let miner_id_bounded = BoundedVec(miner_id.clone());
    // Since there seems to be a bug in subxt that should have been resolved (and we possibly won't have a separate storage map for querying workers by id)
    let miner_registration_confirmation_query = match miner_type {
        MinerType::Cloud => substrate_interface::api::storage()
            .edge_connect()
            .cloud_miners(miner_id_bounded.clone()),
        MinerType::Edge => substrate_interface::api::storage()
            .edge_connect()
            .edge_miners(miner_id_bounded.clone()),
    };

    let result = client
        .storage()
        .at_latest()
        .await?
        .fetch(&miner_registration_confirmation_query)
        .await?;
    if let Some(miner) = result {
        if miner.id.0 == miner_id_bounded.0 {
            println!("Miner successfully registered on-chain!");
            return Ok(RegistrationStatus::Registered(MinerIdentity {
                miner_owner: miner.owner.clone(),
                miner_id: BoundedVec(miner.id.0.clone()),
                miner_type: miner_type,
            }));
        } else {
            println!(
                "Miner ID mismatch — expected {:?}, got {:?}",
                miner_id_bounded.0, miner.id.0
            );
        }
    } else {
        println!("No miner found on-chain for ID {:?}", miner_id_bounded.0);
    }

    Ok(RegistrationStatus::Unknown)
}

pub async fn retrieve_identity(
    keypair: Arc<Keypair>,
    miner_type: Arc<MinerType>,
    miner_uuid: Vec<u8>,
) -> Result<MinerIdentity> {
    let identity: MinerIdentity;
    let value = miner_uuid.clone();

    match confirm_registration().await {
        Ok(RegistrationStatus::Registered(miner_identity)) => {
            println!("Miner is registered, using existing identity.");
            identity = miner_identity;
        }
        Ok(RegistrationStatus::Unknown) => {
            println!("Registration status unknown, attempting registration.");
            identity = pub_register(keypair, miner_type, value).await?;
        }
        Err(e) => {
            println!(
                "Error confirming miner registration: {}, attempting registration.",
                e
            );
            identity = pub_register(keypair, miner_type, value).await?;
        }
    }

    let miner_identity_json = serde_json::to_string(&identity)?;

    update_config_file(&PATHS.identity_path, &miner_identity_json)?;

    Ok(identity)
}

/// Check the miner's current status from the parachain and update accordingly
async fn check_and_update_miner_status(miner: Arc<Miner>) -> Result<()> {
    let client = global_config::get_parachain_client()?;

    // Query operational status from chain
    let operational_status = get_miner_operational_status(
        &client,
        &miner.identity.miner_id,
        &miner.miner_type.as_ref().clone(),
    )
    .await?;

    println!(
        "Miner status from parachain - Operational: {:?}",
        operational_status
    );

    // If we have a task but the operational status is Available, update to Busy
    if miner.current_task.read().await.is_some() {
        if let Some(OperationalStatus::Available) = operational_status {
            println!("Miner has task but operational status is Available, updating to Busy");
            miner
                .update_operational_status(OperationalStatus::Busy)
                .await?;
        }
    } else {
        // If we don't have a task but operational status is Busy, update to Available
        if let Some(OperationalStatus::Busy) = operational_status {
            println!("Miner has no task but operational status is Busy, updating to Available");
            miner
                .update_operational_status(OperationalStatus::Available)
                .await?;
        }
    }

    Ok(())
}

pub async fn start_miner(miner: Arc<Miner>) -> Result<()> {
    println!("Starting miner...");

    // Check current status from parachain and update accordingly
    check_and_update_miner_status(Arc::clone(&miner)).await?;

    println!("Waiting for tasks...");

    let client = global_config::get_parachain_client()?;

    if let Err(e) = pick_up_task(Arc::clone(&miner)).await {
        println!("No task to pick up, performing clean startup: {}", e);
    }

    let mut blocks = client.blocks().subscribe_finalized().await?;

    while let Some(Ok(block)) = blocks.next().await {
        println!("New block imported: {:?}", block.hash());

        let miner_identity = miner.identity.as_ref();

        println!("Active miner identity: {:?}", miner_identity);

        if miner.current_task.read().await.is_none() {
            let mut last_check = LAST_UPDATE_CHECK.lock().await;
            let now = Instant::now();

            // Check if 6 hours have passed since the last update attempt
            if last_check.map_or(true, |t| {
                now.duration_since(t) > Duration::from_secs(24 * 3600)
            }) {
                println!("Miner doesn't have an active task, trying to apply update!");
                if let Err(e) = try_apply_update_if_available() {
                    println!("Update check failed: {:?}", e);
                }
                *last_check = Some(now);
            } else {
                println!("Skipping update check, last checked less than 6 hours ago.");
            }
        }

        let events = block.events().await?;

        for event in events.iter() {
            match event {
                Ok(ev) => {
                    if let Err(e) = miner.process_event(&ev).await {
                        println!("Error processing event: {:?}", e);
                    }
                }
                Err(e) => eprintln!("Error decoding event: {:?}", e),
            }
        }
    }

    Ok(())
}
