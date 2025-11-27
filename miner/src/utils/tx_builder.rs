// Contains all the possible transactions to the parachain, kept out of the `Miner` struct for so that they can contain data that is not the current data (eg. previous taskId)

use crate::error::Error;
use crate::error::Result;
use crate::global_config;
use crate::global_config::CYBORG_NETWORK_URLS;
use crate::specs;
use subxt::config::DefaultExtrinsicParamsBuilder;
use subxt::ext::scale_encode::EncodeAsFields;
use subxt::tx::DefaultPayload;
use subxt::tx::Payload;
use subxt::tx::SubmittableExtrinsic;
use subxt::OnlineClient;
use subxt::PolkadotConfig;
use types::substrate_interface::api::runtime_types::bounded_collections::bounded_vec::BoundedVec;
use types::substrate_interface::{self, api::runtime_types::cyborg_primitives::miner::MinerType};
use types::substrate_interface::api::edge_connect::calls::types::remove_miner::MinerId;
use types::MinerIdentity;
use crate::utils::substrate_queries::get_miner_by_id;
use crate::utils::tx_queue::TxOutput;
use std::fmt::Debug;
use std::sync::Arc;
use types::substrate_interface::api::edge_connect::Error as EdgeConnectError;
use types::substrate_interface::api::neuro_zk::Error as NzkError;
use types::substrate_interface::api::task_management::Error as TaskManagementError;
use subxt_signer::sr25519::Keypair;

pub async fn sign_and_send_tx<T>(
    unsigned_tx: &DefaultPayload<T>,
    miner_uuid: &MinerId,
    miner_type: &MinerType,
    keypair: Option<Arc<Keypair>>, //TODO remove once TEE singer and remote signing serive works
) -> Result<subxt::blocks::ExtrinsicEvents<subxt::PolkadotConfig>, subxt::Error>
    where T: EncodeAsFields
{
    let client = global_config::get_parachain_client()?;

    let result = match miner_type {
        MinerType::Edge => {
            // ----------------------- FUTURE TEE OPERATION -----------------

            println!("WARNING: Transactions are currently not signed by the TEE but by a plaintext key. This is a major vulnerability and needs to be addressed before launching the edge miners!");
            let keypair = keypair
                .ok_or("Cannot sign the tx, no keypair provided!")?;

            let params = DefaultExtrinsicParamsBuilder::new()
                .build();

            let signed_tx = client.tx().
                create_signed(unsigned_tx, keypair.as_ref(), params)
                .await?;

            // ---------------------------------------------------------------
             
            signed_tx.submit_and_watch()
                .await?
                .wait_for_finalized_success()
                .await
        }
        MinerType::Cloud => {
            let req_client = reqwest::Client::builder().build()?;

            let unsigned_bytes = unsigned_tx.encode_call_data(&client.metadata())
                .map_err(|e| Error::Custom(e.to_string()))?;

            let conductor_url = &CYBORG_NETWORK_URLS.conductor;

            let url = format!(
                "{}/sign-tx/{}", 
                conductor_url, 
                String::from_utf8(miner_uuid.0.clone())?
            );
           
            let signed_tx = req_client
                .post(url)
                .header("Content-Type", "application/octet-stream")
                .body(unsigned_bytes)
                .send()
                .await?
                .error_for_status()?
                .bytes()
                .await?
                .to_vec();

            let signed_tx = SubmittableExtrinsic
                ::<PolkadotConfig, OnlineClient<PolkadotConfig>>
                ::from_bytes(client.clone(), signed_tx);
             
            signed_tx.submit_and_watch()
                .await?
                .wait_for_finalized_success()
                .await
        }
    };

    result
}

/// Registers the miner on the blockchain.
///
/// # Returns
/// A `Result` containing a `String` witht the miner identity if successful, or an `Error` if registration fails.
pub async fn register(
    keypair: Arc<Keypair>,
    miner_type: Arc<MinerType>,
    miner_uuid: MinerId,
) -> Result<MinerIdentity> {
    let client = global_config::get_parachain_client()?;

    let worker_specs = specs::gather_worker_spec(Arc::clone(&miner_type)).await?;

    let tx = substrate_interface::api::tx()
        .edge_connect()
        .register_miner(
            miner_type.as_ref().clone(),
            miner_uuid.clone(),
            BoundedVec::from(BoundedVec(worker_specs.domain.clone().as_bytes().to_vec())),
            worker_specs.latitude,
            worker_specs.longitude,
            worker_specs.ram,
            worker_specs.storage,
            worker_specs.cpu,
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
            println!("Miner registration submitted, waiting for transaction to be finalized...");
            e
        })?
        .wait_for_finalized_success()
        .await;

    match tx_submission {
        Ok(e) => {
            let tx_event =
                e.find_first::<substrate_interface::api::edge_connect::events::MinerRegistered>()?;

            if let Some(event) = tx_event {
                println!("Miner registered successfully: {event:?}");

                return Ok(MinerIdentity {
                    miner_owner: event.miner.0.clone(),
                    miner_id: miner_uuid.0.clone(),
                    miner_type: miner_type.as_ref().clone(),
                });
            } else {
                return Err(Error::Custom(
                    "Miner registration event not found, cannot bootstrap miner".to_string(),
                ));
            }
        }
        Err(e) => {
            if let Err(e) = check_for_acceptable_error(&[EdgeConnectError::MinerExists], e) {
                return Err(Error::Custom(e.to_string()));
            } else {
                match get_miner_by_id(client, miner_uuid.clone(), miner_type).await {
                    Ok(miner_identity) => {
                        println!(
                            "Registered miner found: {:?}, {}",
                            miner_identity.miner_id, miner_identity.miner_owner
                        );

                        return Ok(miner_identity);
                    }
                    Err(e) => {
                        return Err(Error::Custom(format!(
                            "UNRECOVERABLE ERROR: Cannot bootstrap miner: {e}"
                        )));
                    }
                }
            };
        }
    }
}

pub async fn pub_register(
    keypair: Arc<Keypair>,
    miner_type: Arc<MinerType>,
    miner_uuid: MinerId,
) -> Result<MinerIdentity> {
    let tx_queue = global_config::get_tx_queue()?;

    let rx = tx_queue
        .enqueue(move || {
            let keypair = Arc::clone(&keypair);
            let miner_type = miner_type.clone();
            let value = miner_uuid.clone();
            async move {
                let result = register(keypair, miner_type, value).await?;
                Ok(TxOutput::RegistrationInfo(result))
            }
        })
        .await?;

    match rx.await {
        Ok(Ok(TxOutput::RegistrationInfo(miner_identity))) => Ok(miner_identity),
        Ok(Err(e)) => Err(format!("Error registering miner: {}", e).into()),
        Err(_) => Err("Response channel dropped.".into()),
        _ => Err("Missing identity string from registration event".into()),
    }
}

/// Submits a zkml (Zero Knowledge Machine Learning) proof to the blockchain.
///
/// # Arguments
/// * `proof` - A `Vec<u8>` containing the zkml proof.
///
/// # Returns
/// A `Result` indicating `Ok(())` if the result is successfully submitted, or an `Error` if it fails.
pub async fn submit_proof(proof: Vec<u8>, keypair: Keypair, current_task: u64) -> Result<()> {
    let proof: BoundedVec<u8> = BoundedVec::from(BoundedVec(proof));

    let client = global_config::get_parachain_client()?;

    let tx = substrate_interface::api::tx()
        .neuro_zk()
        .submit_proof(current_task, proof);

    println!("Transaction Details:");
    println!("Module: {:?}", tx.pallet_name());
    println!("Call: {:?}", tx.call_name());
    println!("Parameters: {:?}", tx.call_data());

    let tx_submission = client
        .tx()
        .sign_and_submit_then_watch_default(&tx, &keypair)
        .await
        .map(|e| {
            println!("Proof submitted, waiting for transaction to be finalized...");
            e
        })?
        .wait_for_finalized_success()
        .await;

    match tx_submission {
        Ok(e) => {
            let tx_event =
                e.find_first::<substrate_interface::api::neuro_zk::events::NzkProofSubmitted>()?;

            if let Some(event) = tx_event {
                println!("Proof submission confirmed: {event:?}");
            } else {
                println!("No proof submission event found!");
            }
        }
        Err(e) => {
            check_for_acceptable_error(&[NzkError::ProofAlreadySubmitted], e)?;
        }
    }

    Ok(())
}

async fn confirm_task_reception(keypair: Arc<Keypair>, current_task: &u64) -> Result<()> {
    let client = global_config::get_parachain_client()?;

    let tx = substrate_interface::api::tx()
        .task_management()
        .confirm_task_reception(*current_task);

    println!("Transaction Details:");
    println!("Module: {:?}", tx.pallet_name());
    println!("Call: {:?}", tx.call_name());
    println!("Parameters: {:?}", tx.call_data());

    let tx_submission = client
        .tx()
        .sign_and_submit_then_watch_default(&tx, keypair.as_ref())
        .await
        .map(|e| {
            println!(
                "Task reception confirmation submitted, waiting for transaction to be finalized..."
            );
            e
        })?
        .wait_for_finalized_success()
        .await;

    match tx_submission {
        Ok(e) => {
            let event = e
                .find_first::<substrate_interface::api::task_management::events::TaskReceptionConfirmed>(
            )?;

            if let Some(event) = event {
                println!("Task reception confirmed: {event:?}");
            } else {
                println!("No task reception event found!");
            }
        }
        Err(e) => {
            check_for_acceptable_error(
                &[
                    TaskManagementError::RequireAssignedTask,
                    TaskManagementError::TaskReceptionAlreadyConfirmed,
                ],
                e,
            )?;
        }
    }

    Ok(())
}

pub async fn pub_confirm_task_reception(
    keypair: Arc<Keypair>,
    current_task_id: &u64,
) -> Result<()> {
    let tx_queue = global_config::get_tx_queue()?;
    let current_task_id_copy = *current_task_id;

    let rx = tx_queue
        .enqueue(move || {
            let keypair = Arc::clone(&keypair);
            async move {
                let _ = confirm_task_reception(keypair, &current_task_id_copy).await?;
                Ok(TxOutput::Success)
            }
        })
        .await?;

    match rx.await {
        Ok(Ok(TxOutput::Success)) => println!("Task reception confirmed"),
        Ok(Err(e)) => println!("Error confirming task reception: {}", e),
        _ => println!("Unexpected response for task confirmation"),
    }

    Ok(())
}

/// Vacates a miner erasing current user data and resetting the miner state.
///
/// # Returns
/// A `Result` indicating `Ok(())` if the session vacates successfully, or an `Error` if it fails.
pub async fn confirm_miner_vacation(
    keypair: Arc<Keypair>,
    task_id: u64,
    miner_type: Arc<MinerType>,
) -> Result<()> {
    let client = global_config::get_parachain_client()?;

    let tx = substrate_interface::api::tx()
        .task_management()
        .confirm_miner_vacation(task_id, miner_type.as_ref().clone());

    println!("Transaction Details:");
    println!("Module: {:?}", tx.pallet_name());
    println!("Call: {:?}", tx.call_name());
    println!("Parameters: {:?}", tx.call_data());

    let tx_submission = client
        .tx()
        .sign_and_submit_then_watch_default(&tx, keypair.as_ref())
        .await
        .map(|e| {
            println!(
                "Miner vacation confirmation submitted, waiting for transaction to be finalized..."
            );
            e
        })?
        .wait_for_finalized_success()
        .await;

    match tx_submission {
        Ok(e) => {
            let tx_event =
                e.find_first::<substrate_interface::api::task_management::events::MinerVacated>()?;

            if let Some(event) = tx_event {
                println!("Miner vacation confirmed: {event:?}");
            } else {
                println!("No miner vacation event found!");
            }
        }
        Err(e) => {
            check_for_acceptable_error(&[TaskManagementError::InvalidTaskState], e)?;
        }
    }

    Ok(())
}

// This takes in a generic that implements debug as the errors that will be put in here are different types of errors
/// Lets acceptable errors pass through so that the transaction queue doesn't repeat them, because the transaction already succeeded. In some cases for example, the parachain
/// will accept a transaction, but return an error anyway which will cause the transaction queue to re-queue the transaction. Upon trying again, the transaction will be rejected again,
/// because the transaction DID already succeed previously. The function is a workaround for this. It checks the returned error and if it is an error of this sort it lets it pass,
/// causing the transaction queue to not re-queue the transaction.
fn check_for_acceptable_error<T: Debug>(expected_errors: &[T], e: subxt::Error) -> Result<()> {
    match e {
        subxt::Error::Runtime(err) => {
            match err {
                subxt::error::DispatchError::Module(returned_error) => {
                    let returned_error_details = returned_error
                        .details()
                        .map_err(|err| Error::Custom(err.to_string()))?;

                    let returned_error_string = returned_error_details.variant.name.to_string();

                    println!(
                        "Error details - returned error: {:?}",
                        returned_error_string
                    );

                    for expected_error in expected_errors {
                        let expected_error_string = format!("{:?}", expected_error);
                        println!(
                            "Error details - expected error: {:?}",
                            expected_error_string
                        );

                        if returned_error_string == expected_error_string {
                            return Ok(());
                        }
                    }

                    return Err(Error::Custom(returned_error.to_string()));
                }
                _ => return Err(Error::Custom(err.to_string())),
            };
        }
        _ => return Err(e.into()),
    }
}
