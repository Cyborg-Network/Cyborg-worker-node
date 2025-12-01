// Contains all the possible transactions to the parachain, kept out of the `Miner` struct for so that they can contain data that is not the current data (eg. previous taskId)

use crate::error::Error;
use crate::error::{Result as CyborgResult};
use crate::global_config;
use crate::global_config::CYBORG_NETWORK_URLS;
use crate::specs;
use subxt::config::DefaultExtrinsicParamsBuilder;
use subxt::ext::scale_encode::EncodeAsFields;
use subxt::tx::DefaultPayload;
use subxt::utils::{AccountId32, MultiAddress};
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
use subxt_signer::sr25519::{Keypair, Signature};

pub async fn sign_and_send_tx<T>(
    unsigned_tx: &DefaultPayload<T>,
    miner_uuid: &MinerId,
    miner_type: &MinerType,
    miner_owner: AccountId32,
    keypair: Option<Arc<Keypair>>, //TODO remove once TEE singer and remote signing serive works
) -> CyborgResult<Result<subxt::blocks::ExtrinsicEvents<subxt::PolkadotConfig>, subxt::Error>>
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

            let partial_tx = client
                .tx()
                .create_partial_signed(
                    unsigned_tx, 
                    &miner_owner, 
                    Default::default(),
                )
                .await?;

            let signer_payload_bytes = partial_tx.signer_payload();

            let conductor_url = &CYBORG_NETWORK_URLS.conductor;

            let url = format!(
                "{}/sign-tx/{}", 
                conductor_url, 
                String::from_utf8(miner_uuid.0.clone())?
            );

            let signature_bytes: [u8; 64] = req_client
                .post(url)
                .header("Content-Type", "application/octet-stream")
                .body(signer_payload_bytes)
                .send()
                .await?
                .error_for_status()?
                .bytes()
                .await?
                .to_vec()
                .try_into()
                .map_err(|_| "Failed to turn signature response into signature bytes")?;

            let signature = Signature(signature_bytes);

            let multi_addr = MultiAddress::Id(miner_owner.clone());
            let signed_tx = partial_tx
                .sign_with_address_and_signature(&multi_addr, &signature.into());
             
            signed_tx.submit_and_watch()
                .await?
                .wait_for_finalized_success()
                .await
        }
    };

    // Even though we return Ok(result) here, the return type STILL is a result so that the subxt
    // success or error can be analyzed by the pre-existing structure
    Ok(result)
}

/// Registers the miner on the blockchain.
///
/// # Returns
/// A `Result` containing a `String` witht the miner identity if successful, or an `Error` if registration fails.
pub async fn register(
    keypair: Arc<Keypair>,
    miner_type: Arc<MinerType>,
    miner_uuid: MinerId,
) -> CyborgResult<MinerIdentity> {
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
) -> CyborgResult<MinerIdentity> {
    let tx_queue = global_config::get_tx_queue()?;

    let rx = tx_queue
        .enqueue(move || {
            let keypair = Arc::clone(&keypair);
            let miner_type = miner_type.clone();
            let miner_uuid = miner_uuid.clone();
            async move {
                let result = register(keypair, miner_type, miner_uuid).await?;
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
pub async fn submit_proof(
    current_task: u64,
    proof: Vec<u8>, 
    keypair: Arc<Keypair>, 
    miner_type: Arc<MinerType>,
    miner_uuid: MinerId,
    miner_owner: AccountId32
) -> CyborgResult<()> {
    let proof: BoundedVec<u8> = BoundedVec::from(BoundedVec(proof));

    let tx = substrate_interface::api::tx()
        .neuro_zk()
        .submit_proof(current_task, proof);

    println!("Transaction Details:");
    println!("Module: {:?}", tx.pallet_name());
    println!("Call: {:?}", tx.call_name());
    println!("Parameters: {:?}", tx.call_data());

    let tx_submission = sign_and_send_tx(&tx, &miner_uuid, &miner_type, miner_owner, Some(keypair)).await?;
    
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

async fn confirm_task_reception(
    current_task: &u64,
    keypair: Arc<Keypair>, 
    miner_type: Arc<MinerType>,
    miner_uuid: MinerId,
    miner_owner: AccountId32,
) -> CyborgResult<()> {
    let tx = substrate_interface::api::tx()
        .task_management()
        .confirm_task_reception(*current_task);

    println!("Transaction Details:");
    println!("Module: {:?}", tx.pallet_name());
    println!("Call: {:?}", tx.call_name());
    println!("Parameters: {:?}", tx.call_data());

    let tx_submission = sign_and_send_tx(&tx, &miner_uuid, &miner_type, miner_owner, Some(keypair)).await?;

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
    current_task_id: &u64,
    keypair: Arc<Keypair>,
    miner_type: Arc<MinerType>,
    miner_uuid: MinerId,
    miner_owner: AccountId32,
) -> CyborgResult<()> {
    let tx_queue = global_config::get_tx_queue()?;
    let current_task_id_copy = *current_task_id;

    let rx = tx_queue
        .enqueue(move || {
            let keypair = Arc::clone(&keypair);
            let miner_type = Arc::clone(&miner_type);
            let miner_uuid = miner_uuid.clone();
            let miner_owner = miner_owner.clone();
            async move {
                let _ = confirm_task_reception(
                    &current_task_id_copy, 
                    keypair, 
                    miner_type, 
                    miner_uuid,
                    miner_owner
                ).await?;
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
    task_id: u64,
    miner_type: Arc<MinerType>,
    keypair: Arc<Keypair>,
    miner_uuid: MinerId,
    miner_owner: AccountId32,
) -> CyborgResult<()> {
    let tx = substrate_interface::api::tx()
        .task_management()
        .confirm_miner_vacation(task_id, miner_type.as_ref().clone());

    println!("Transaction Details:");
    println!("Module: {:?}", tx.pallet_name());
    println!("Call: {:?}", tx.call_name());
    println!("Parameters: {:?}", tx.call_data());

    let tx_submission = sign_and_send_tx(&tx, &miner_uuid, &miner_type, miner_owner, Some(keypair)).await?;

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
fn check_for_acceptable_error<T: Debug>(expected_errors: &[T], e: subxt::Error) -> CyborgResult<()> {
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
