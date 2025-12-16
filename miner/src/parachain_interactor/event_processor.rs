use crate::utils::task_handling::{self, return_task_container_name, set_current_task};
use crate::utils::tx_builder::pub_confirm_task_reception;
use crate::{
    error::{Error, Result},
    miner_types::Miner,
};
use std::sync::Arc;
use subxt::{events::EventDetails, PolkadotConfig};
use types::substrate_interface::api::runtime_types::cyborg_primitives::miner::OperationalStatus;
use types::CurrentTask;
use types::{substrate_interface, TaskPreparationStatus};

use super::registration::update_operational_status;

pub async fn process_event(miner: Arc<Miner>, event: &EventDetails<PolkadotConfig>) -> Result<()> {
    // Extract the task_id before matching to avoid having to clone miner
    let current_task_id = {
        let guard = miner.current_task.read().await;
        guard.as_ref().map(|t| t.id)
    };

    // Check for MinerRegistered event
    match event.as_event::<substrate_interface::api::edge_connect::events::MinerRegistered>() {
        Ok(Some(miner_registered)) => {
            let creator = &miner_registered.creator;
            let miner = &miner_registered.miner;
            let domain = &miner_registered.domain;

            println!(
                "Miner Registered: Creator: {:?}, Miner: {:?}, Domain: {:?}",
                creator, miner, domain
            );
        }
        Err(e) => {
            println!("Error decoding MinerRegistered event: {:?}", e);
            return Err(Error::Subxt(Box::new(subxt::Error::from(e))));
        }
        _ => {} // Skip non-matching events
    }

    // Check for MinerRemoved event
    match event.as_event::<substrate_interface::api::edge_connect::events::MinerRemoved>() {
        Ok(Some(miner_removed)) => {
            let creator = &miner_removed.creator;
            let miner_id = &miner_removed.miner_id;

            println!(
                "Miner Removed: Creator: {:?}, Miner ID: {:?}",
                creator, miner_id
            );
        }
        Err(e) => {
            println!("Error decoding MinerRemoved event: {:?}", e);
            return Err(Error::Subxt(Box::new(subxt::Error::from(e))));
        }
        _ => {} // Skip non-matching events
    }

    // Check for OracleStatusUpdated event
    match event.as_event::<substrate_interface::api::edge_connect::events::OracleStatusUpdated>() {
        Ok(Some(status_updated)) => {
            let worker = &status_updated.worker;
            let online = &status_updated.online;

            println!(
                "Oracle Status Updated: Worker: {:?}, Online: {:?}",
                worker, online
            );
        }
        Err(e) => {
            println!("Error decoding OracleStatusUpdated event: {:?}", e);
            return Err(Error::Subxt(Box::new(subxt::Error::from(e))));
        }
        _ => {} // Skip non-matching events
    }

    // Check for OperationalStatusUpdated event
    match event
        .as_event::<substrate_interface::api::edge_connect::events::OperationalStatusUpdated>()
    {
        Ok(Some(status_updated)) => {
            let worker = &status_updated.worker;
            let status = &status_updated.status;

            println!(
                "Operational Status Updated: Worker: {:?}, Status: {:?}",
                worker, status
            );
        }
        Err(e) => {
            println!("Error decoding OperationalStatusUpdated event: {:?}", e);
            return Err(Error::Subxt(Box::new(subxt::Error::from(e))));
        }
        _ => {} // Skip non-matching events
    }

    // Check for TaskScheduled event
    match event.as_event::<substrate_interface::api::task_management::events::TaskScheduled>() {
        Ok(Some(task_scheduled)) => {
            let assigned_miner = &task_scheduled.assigned_miner;

            if assigned_miner.0 == miner.identity.miner_id {
                println!("New task scheduled: {:?}", task_scheduled.task_id);

                update_operational_status(Arc::clone(&miner), OperationalStatus::Busy).await?;

                let (status_tx, mut status_rx) =
                    tokio::sync::watch::channel(TaskPreparationStatus::Preparing);
                let current_task = CurrentTask {
                    task_owner: task_scheduled.task_owner,
                    task_type: task_scheduled.task_kind,
                    container_name: return_task_container_name(task_scheduled.task_id),
                    id: task_scheduled.task_id,
                    status_sender: status_tx,
                };

                let (current_task_id, _handle) =
                    set_current_task(Arc::clone(&miner), current_task).await?;

                if let Ok(status_ref) = status_rx
                    .wait_for(|status| *status != TaskPreparationStatus::Preparing)
                    .await
                {
                    let status = status_ref.clone();
                    let keypair = Arc::clone(&miner.keypair);
                    tokio::spawn(async move {
                        if let Err(e) =
                            pub_confirm_task_reception(keypair, &current_task_id, status).await
                        {
                            println!(
                                "Critical error encountered, please contact the support: {}",
                                e
                            );
                        }
                    });
                };
            }
        }
        Err(e) => {
            println!("Error decoding TaskScheduled event: {:?}", e);
            return Err(Error::Subxt(Box::new(subxt::Error::from(e))));
        }
        _ => {} // Skip non-matching events
    }

    if let Some(current_task_id) = current_task_id {
        match event
            .as_event::<substrate_interface::api::task_management::events::TaskStopRequested>()
        {
            Ok(Some(requested_task_stop)) => {
                let task_id = &requested_task_stop.task_id;

                if *task_id == current_task_id {
                    task_handling::clean_up_current_task_and_vacate(Arc::clone(&miner)).await?;
                }
            }
            Err(e) => {
                println!("Error decoding TaskStopRequested event: {:?}", e);
                return Err(Error::Subxt(Box::new(subxt::Error::from(e))));
            }
            _ => {} // Skip non-matching events
        }
    }

    Ok(())
}
