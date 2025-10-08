use crate::global_config::{PATHS, self, update_config_file};
use crate::error::Result;
use crate::self_update::try_apply_update_if_available;
use crate::substrate_interface;
use crate::substrate_interface::api::runtime_types::cyborg_primitives::miner::{MinerType, OperationalStatus};
use crate::utils::task_handling::pick_up_task;
use crate::traits::ParachainInteractor;
use crate::types::{Miner, MinerIdentity};
use crate::utils::tx_builder::pub_register;
use subxt_signer::sr25519::Keypair;
use std::fs;
use std::sync::Arc;

pub enum RegistrationStatus{
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
    let identity = identity.miner_id;

    println!("Confirming miner registration...");

    println!("identity: {:?}", identity);

    // Since there seems to be a bug in subxt that should have been resolved (and we possibly won't have a separate storage map for querying workers by id)
    let miner_registration_confirmation_query = match miner_type {
        MinerType::Cloud => {
            substrate_interface::api::storage()
                .edge_connect()
                .cloud_miners_iter()
        }
        MinerType::Edge => {
            substrate_interface::api::storage()
                .edge_connect()
                .edge_miners_iter()
        }
    };

    let mut result = client
        .storage()
        .at_latest()
        .await?
        .iter(miner_registration_confirmation_query)
        .await?;

    while let Some(Ok(miner)) = result.next().await {
        if miner.value.owner == identity.0 && miner.value.id == identity.1 {
            return Ok(RegistrationStatus::Registered(
                MinerIdentity {
                    miner_owner: miner.value.owner.clone(),
                    miner_id: (miner.value.owner, miner.value.id),
                    miner_type: miner_type,
                }
            ));
        }
    }

    println!("Miner is not registered");
    Ok(RegistrationStatus::Unknown)
}

pub async fn retrieve_identity(keypair: Arc<Keypair>, miner_type: Arc<MinerType>) -> Result<MinerIdentity> {
    let identity: MinerIdentity;

    match confirm_registration().await {
        Ok(RegistrationStatus::Registered(miner_identity)) => {
            println!("Miner is registered, using existing identity.");
            identity = miner_identity;
        }, 
        Ok(RegistrationStatus::Unknown) => {
            println!("Registration status unknown, attempting registration.");
            identity = pub_register(keypair, miner_type).await?;
        },
        Err(e) => {
            println!("Error confirming miner registration: {}, attempting registration.", e);
            identity = pub_register(keypair, miner_type).await?;
        }
    }

    let miner_identity_json = serde_json::to_string(&identity)?;

    update_config_file(&PATHS.identity_path, &miner_identity_json)?;

    Ok(identity)
}

// Add new function to update operational status
pub async fn update_operational_status(miner: Arc<Miner>, status: OperationalStatus) -> Result<()> {
    let client = global_config::get_parachain_client()?;
    
    let tx = substrate_interface::api::tx()
        .edge_connect()
        .update_operational_status(
            miner.miner_type.as_ref().clone(),
            miner.identity.miner_id.1,
            status
        );

    println!("Updating operational status to: {:?}", status);
    
    let _ = client
        .tx()
        .sign_and_submit_then_watch_default(&tx, miner.keypair.as_ref())
        .await?
        .wait_for_finalized_success()
        .await?;

    println!("Operational status updated successfully");
    Ok(())
}

pub async fn start_miner(miner: Arc<Miner>) -> Result<()> {
    println!("Starting miner...");

    // Set operational status to Available when starting
    update_operational_status(Arc::clone(&miner), OperationalStatus::Available).await?;

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
            println!("Miner doesn't have an active task, trying to apply update!");
            try_apply_update_if_available()?;
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