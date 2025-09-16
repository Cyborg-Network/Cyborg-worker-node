use crate::config::get_paths;
use crate::substrate_interface;
use crate::traits::{InferenceServer, ParachainInteractor};
use crate::types::CurrentTask;
use crate::utils::task_handling::{self, return_task_container_name, set_current_task};
use crate::utils::tx_builder::pub_confirm_task_reception;
use crate::{
    error::{Error, Result},
    types::{Miner, MinerData},
};
use subxt::{events::EventDetails, PolkadotConfig};
use std::fs;

pub async fn process_event(miner: &mut Miner, event: &EventDetails<PolkadotConfig>) -> Result<()> {
    // Check for WorkerRegistered event
    match event.as_event::<substrate_interface::api::edge_connect::events::WorkerRegistered>() {
        Ok(Some(worker_registered)) => {
            let creator = &worker_registered.creator;
            let worker = &worker_registered.worker;
            let domain = &worker_registered.domain;

            println!(
                "Worker Registered: Creator: {:?}, Worker: {:?}, Domain: {:?}",
                creator, worker, domain
            );
        }
        Err(e) => {
            println!("Error decoding WorkerRegistered event: {:?}", e);
            return Err(Error::Subxt(e.into()));
        }
        _ => {} // Skip non-matching events
    }

    // Check for WorkerRemoved event
    match event.as_event::<substrate_interface::api::edge_connect::events::WorkerRemoved>() {
        Ok(Some(worker_removed)) => {
            let creator = &worker_removed.creator;
            let worker_id = &worker_removed.worker_id;

            println!(
                "Worker Removed: Creator: {:?}, Worker ID: {:?}",
                creator, worker_id
            );
        }
        Err(e) => {
            println!("Error decoding WorkerRemoved event: {:?}", e);
            return Err(Error::Subxt(e.into()));
        }
        _ => {} // Skip non-matching events
    }

    // Check for WorkerStatusUpdated event
    match event.as_event::<substrate_interface::api::edge_connect::events::WorkerStatusUpdated>() {
        Ok(Some(status_updated)) => {
            let creator = &status_updated.creator;
            let worker_id = &status_updated.worker_id;
            let worker_status = &status_updated.worker_status;

            println!(
                "Worker Status Updated: Creator: {:?}, Worker ID: {:?}, Status: {:?}",
                creator, worker_id, worker_status
            );
        }
        Err(e) => {
            println!("Error decoding WorkerStatusUpdated event: {:?}", e);
            return Err(Error::Subxt(e.into()));
        }
        _ => {} // Skip non-matching events
    }

    // Check for TaskScheduled event
    match event.as_event::<substrate_interface::api::task_management::events::TaskScheduled>() {
        Ok(Some(task_scheduled)) => {
            let assigned_miner = &task_scheduled.assigned_worker;
            let identity_path = &get_paths()?.identity_path;

            let file_content = fs::read_to_string(identity_path)?;
            let miner_data: MinerData = serde_json::from_str(&file_content)?;

            if assigned_miner == &miner_data.miner_identity {
                println!("New task scheduled: {:?}", task_scheduled.task_id);

                let current_task = CurrentTask {
                    task_owner: task_scheduled.task_owner,
                    task_type: task_scheduled.task_kind,
                    container_name: return_task_container_name(task_scheduled.task_id),
                    id: task_scheduled.task_id,
                };

                let (current_task_id, _handle) = set_current_task(miner, current_task).await?;

                
                let keypair = miner.keypair.clone();
                tokio::spawn(async move {
                    if let Err(e) = pub_confirm_task_reception(keypair, &current_task_id).await {
                        println!("Critical error encountered, please contact the support: {}", e);
                    }
                });

            }
        }
        Err(e) => {
            println!("Error decoding TaskScheduled event: {:?}", e);
            return Err(Error::Subxt(e.into()));
        }
        _ => {} // Skip non-matching events
    }

    if let Some(current_task) = &miner.current_task {
        let current_task_id = current_task.id;

        match event.as_event::<substrate_interface::api::task_management::events::TaskStopRequested>() {
            Ok(Some(requested_task_stop)) => {
                let task_id = &requested_task_stop.task_id;

                if *task_id == current_task_id {
                    task_handling::clean_up_current_task_and_vacate(miner).await?;
                }
            }
            Err(e) => {
                println!("Error decoding TaskStopRequested event: {:?}", e);
                return Err(Error::Subxt(e.into()));
            }
            _ => {} // Skip non-matching events
        }
    }

    if let Some(current_task) = &miner.current_task {
        match event.as_event::<substrate_interface::api::neuro_zk::events::NzkProofRequested>() {
            Ok(Some(requested_proof)) => {
                let task_id = &requested_proof.task_id;

                if *task_id == current_task.id {
                    let proof = miner.parent_runtime.read().await.generate_proof().await?;
                    let _ = miner.submit_zkml_proof(proof).await?;
                }
            }
            Err(e) => {
                println!("Error decoding SubmittedCompletedTask event: {:?}", e);
                return Err(Error::Subxt(e.into()));
            }
            _ => {} // Skip non-matching events
        }
    }

    /*
    //TODO check if proof was submitted (after parachain update)
    // Check for SubmittedCompletedTask event to check if worker was assigned to verify task
    match event.as_event::<substrate_interface::api::neuro_zk::events::ProofSubmitted>() {
        Ok(Some(submitted_proof)) => {
            let prover = &submitted_task.prover;

            if *prover == self.identity {
                //TODO add an proof submission state somewhere that tracks if the proof was submitted or not (wait 60sec otherwise retry)
                //TODO set the above mentioned state to submitted
            }
        }
        Err(e) => {
            println!("Error decoding SubmittedCompletedTask event: {:?}", e);
            return Err(Error::Subxt(e.into()));
        }
        _ => {} // Skip non-matching events
    }
    */

    Ok(())
}
