use crate::error::Result;
use crate::types::Miner;
use crate::{global_config, substrate_interface};

pub async fn _miner_self_suspend(miner: &Miner) -> Result<()> {
    let client = global_config::get_parachain_client()?;
    let miner_id = &miner.identity.miner_id;

    // TODO This needs a special function and miners need a quarantine or other way to punish suspicious behavior
    let worker_suspension = substrate_interface::api::tx()
        .edge_connect()
        .toggle_miner_visibility(miner.miner_type.as_ref().clone(), miner_id.1, false);

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
        .find_first::<substrate_interface::api::edge_connect::events::MinerStatusUpdated>(
    )?;

    if let Some(event) = suspension_event {
        println!("Miner suspended successfully: {event:?}");
    } else {
        println!("Miner suspension failed");
    }

    Ok(())
}
