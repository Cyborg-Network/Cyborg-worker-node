use std::sync::Arc;

use crate::error::Result;
use crate::substrate_interface::api::runtime_types::cyborg_primitives::miner::OperationalStatus;
use crate::types::Miner;
use crate::{global_config, substrate_interface};

pub async fn _miner_self_suspend(miner: &Miner) -> Result<()> {
    let client = global_config::get_parachain_client()?;
    let miner_id = &miner.identity.miner_id;

    // TODO This needs a special function and miners need a quarantine or other way to punish suspicious behavior
    let worker_suspension = substrate_interface::api::tx()
        .edge_connect()
        .update_operational_status(miner.miner_type.as_ref().clone(), miner_id.1, OperationalStatus::Suspended);

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

/// Updates the operational status on the parachain (non-blocking)
pub async fn update_operational_status(miner: Arc<Miner>, status: OperationalStatus) -> Result<()> {
    let current_status = miner.get_operational_status().await;

    // Only update if status has changed
    if matches!(
        (&current_status, &status),
        (OperationalStatus::Available, OperationalStatus::Available)
            | (OperationalStatus::Busy, OperationalStatus::Busy)
            | (OperationalStatus::Suspended, OperationalStatus::Suspended)
    ) {
        return Ok(());
    }

    // Clone the status before moving into the async block
    let status_for_spawn = status.clone();
    let keypair = Arc::clone(&miner.keypair);
    let miner_type = Arc::clone(&miner.miner_type);
    let miner_id = miner.identity.miner_id.1;

    tokio::spawn(async move {
        if let Err(e) = async {
            let tx_queue = global_config::get_tx_queue()?;

            // Clone status for the closure
            let status_for_closure = status_for_spawn.clone();
            let rx = tx_queue.enqueue(move || {
                let keypair = Arc::clone(&keypair);
                let miner_type = Arc::clone(&miner_type);
                let status_inner = status_for_closure.clone();

                async move {
                    let client = global_config::get_parachain_client()?;

                    // Print the status before moving it
                    println!("Updating operational status to: {:?}", status_inner);

                    let tx = substrate_interface::api::tx()
                        .edge_connect()
                        .update_operational_status(
                            miner_type.as_ref().clone(),
                            miner_id,
                            status_inner,
                        );

                    println!("Transaction Details:");
                    println!("Module: {:?}", tx.pallet_name());
                    println!("Call: {:?}", tx.call_name());
                    println!("Parameters: {:?}", tx.call_data());

                    let tx_submission = client
                        .tx()
                        .sign_and_submit_then_watch_default(&tx, keypair.as_ref())
                        .await
                        .map(|e| {
                            println!("Operational status update submitted, waiting for transaction to be finalized...");
                            e
                        })?
                        .wait_for_finalized_success()
                        .await;

                    match tx_submission {
                        Ok(e) => {
                            let tx_event = e.find_first::<
                                substrate_interface::api::edge_connect::events::OperationalStatusUpdated,
                            >()?;

                            if let Some(event) = tx_event {
                                println!("Operational status updated successfully: {event:?}");
                            } else {
                                println!("No operational status update event found!");
                            }
                        }
                        Err(e) => {
                            // Check for acceptable errors
                            check_for_acceptable_operational_status_error(e)?;
                            println!("Operational status update completed (acceptable error)");
                        }
                    }

                    Ok(crate::utils::tx_queue::TxOutput::Success)
                }
            })
            .await?;

            match rx.await {
                Ok(Ok(crate::utils::tx_queue::TxOutput::Success)) => {
                    println!("Operational status updated successfully");
                }
                Ok(Ok(crate::utils::tx_queue::TxOutput::RegistrationInfo(_))) => {
                    // This should never happen for operational status updates, but handle it
                    println!("Unexpected RegistrationInfo in operational status update response");
                }
                Ok(Err(e)) => {
                    println!("Error updating operational status: {}", e);
                }
                Err(_) => {
                    println!("Response channel dropped for operational status update");
                }
            }

            Ok::<_, crate::error::Error>(())
        }
        .await
        {
            println!("Error in operational status update task: {}", e);
        }
    });

    // Update local status cache
    miner.set_operational_status(status).await;

    Ok(())
}

/// Check for acceptable errors when updating operational status
fn check_for_acceptable_operational_status_error(e: subxt::Error) -> Result<()> {
    match e {
        subxt::Error::Runtime(err) => {
            match err {
                subxt::error::DispatchError::Module(returned_error) => {
                    let returned_error_details = returned_error
                        .details()
                        .map_err(|err| crate::error::Error::Custom(err.to_string()))?;

                    let returned_error_string = returned_error_details.variant.name.to_string();

                    // Acceptable errors for operational status updates
                    let acceptable_errors = [
                        "MinerDoesNotExist", // Miner might have been removed
                        "NotAuthorized",     // Permission issues (temporary)
                        "MinerSuspended",    // Miner is suspended (temporary state)
                    ];

                    for acceptable_error in &acceptable_errors {
                        if returned_error_string == *acceptable_error {
                            return Ok(());
                        }
                    }

                    // If not an acceptable error, propagate it
                    return Err(crate::error::Error::Custom(returned_error.to_string()));
                }
                _ => return Err(crate::error::Error::Custom(err.to_string())),
            };
        }
        _ => return Err(e.into()),
    }
}