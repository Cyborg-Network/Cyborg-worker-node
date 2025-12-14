use types::substrate_interface;

use std::sync::Arc;

use crate::{
    error::Result,
    global_config,
    miner_types::Miner,
};

pub async fn confirm_task_reception(miner: Arc<Miner>) -> Result<()> {
    let client = global_config::get_parachain_client()?;
    let current_task = miner.current_task().await?.read().await.id;

    let task_confirmation = substrate_interface::api::tx()
        .task_management()
        .confirm_task_reception(current_task, false);

    println!("Transaction Details:");
    println!("Module: {:?}", task_confirmation.pallet_name());
    println!("Call: {:?}", task_confirmation.call_name());
    println!("Parameters: {:?}", task_confirmation.call_data());

    let worker_registration_events = client
        .tx()
        .sign_and_submit_then_watch_default(&task_confirmation, miner.keypair.as_ref())
        .await
        .map(|e| {
            println!(
                "Task reception confirmation submitted, waiting for transaction to be finalized..."
            );
            e
        })?
        .wait_for_finalized_success()
        .await?;

    let registration_event = worker_registration_events
        .find_first::<substrate_interface::api::task_management::events::TaskReceptionConfirmed>(
    )?;

    if let Some(event) = registration_event {
        println!("Task reception confirmed: {event:?}");
    } else {
        println!("Task reception confirmation failed!");
    }

    Ok(())
}
