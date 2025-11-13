use types::substrate_interface::api::runtime_types::bounded_collections::bounded_vec::BoundedVec;
use types::substrate_interface::api::runtime_types::cyborg_primitives::miner::OperationalStatus;
use types::substrate_interface;

use crate::miner_types::Miner;
use crate::error::Result;
use crate::global_config;

pub async fn _miner_self_suspend(miner: &Miner) -> Result<()> {
    let client = global_config::get_parachain_client()?;
    let miner_id = &miner.identity.miner_id;
    let miner_id_bounded = BoundedVec(miner_id.0.clone());

    // TODO This needs a special function and miners need a quarantine or other way to punish suspicious behavior
    let worker_suspension = substrate_interface::api::tx()
        .edge_connect()
        .update_operational_status(
            miner.miner_type.as_ref().clone(),
            miner_id_bounded,
            OperationalStatus::Suspended,
        );

    println!("Transaction Details:");
    println!("Module: {:?}", worker_suspension.pallet_name());
    println!("Call: {:?}", worker_suspension.call_name());
    println!("Parameters: {:?}", worker_suspension.call_data());

    let miner_suspension_events = client
        .tx()
        .sign_and_submit_then_watch_default(&worker_suspension, miner.keypair.as_ref())
        .await
        .map(|e| {
            println!("Miner suspension submitted, waiting for transaction to be finalized...");
            e
        })?
        .wait_for_finalized_success()
        .await?;

    let suspension_event = miner_suspension_events
        .find_first::<substrate_interface::api::edge_connect::events::OperationalStatusUpdated>(
    )?;

    if let Some(event) = suspension_event {
        println!("Miner suspended successfully: {event:?}");
    } else {
        println!("Miner suspension failed");
    }

    Ok(())
}
