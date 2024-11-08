use async_trait::async_trait;
use pinata_sdk::PinByJson;
use pinata_sdk::PinataApi;
use sp_core::blake2_256;
use std::error::Error;
use std::io::Read;
use std::process::Output;
use std::str::FromStr;
use subxt::events::EventDetails;
use subxt::utils::H256;
use zbus::{Connection, Message};
//use subxt::ext::jsonrpsee::async_client::ClientBuilder;

use codec::{Decode, Encode};
use log::info;
//use sc_client_api::BlockchainEvents;
use serde::{Deserialize, Serialize};
//use sp_api::ProvideRuntimeApi;
//use sp_blockchain::HeaderBackend;
//use sp_runtime::traits::Block;
use subxt::utils::AccountId32;
use std::fs;
use subxt::{OnlineClient, PolkadotConfig};
use subxt_signer::sr25519::Keypair;
use substrate_interface::api::runtime_types::bounded_collections::bounded_vec::BoundedVec;
use reqwest::get;
use std::fs::File;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::{Command, Stdio};
use zip::read::ZipArchive;

use crate::{substrate_interface, specs};

pub const CONFIG_FILE_NAME: &str = "worker_config.json";
pub const WORK_PACKAGE_DIR: &str = "current_task";

// datastructure for worker registartion persistence
#[derive(Debug, Clone, PartialEq, Eq, Decode, Encode, Serialize, Deserialize)]
pub struct WorkerData {
    pub creator: String,
    pub worker: (AccountId32, u64),
}

pub struct WorkerConfig {
    pub domain: BoundedVec<u8>,
    pub latitude: i32,
    pub longitude: i32,
    pub ram: u64,
    pub storage: u64,
    pub cpu: u16,
}

#[derive(Deserialize)]
pub struct IpResponse {
    pub ip: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ZkFiles {
    zk_public_input: Option<String>,
    zk_circuit: Option<String>,
}

#[async_trait]
/// A trait for blockchain client operations, such as registering a worker, starting mining sessions, and processing events.
///
/// Provides an asynchronous API for interacting with a blockchain, which enables clients to register workers,
/// initiate mining sessions, and handle blockchain events with asynchronous operations.
pub trait BlockchainClient {
    /// Registers a worker node on the blockchain.
    ///
    /// # Returns
    /// A `Result` indicating `Ok(())` if successful, or an `Error` if registration fails.
    async fn register_worker(&self) -> Result<(), Box<dyn Error>>;

    /// Starts a mining session on the blockchain by subscribing to events and listening to finalized blocks.
    ///
    /// # Returns
    /// A `Result` indicating `Ok(())` if the session starts successfully, or an `Error` if it fails.
    async fn start_mining_session(&self) -> Result<(), Box<dyn Error>>;

    /// Processes an event received from the blockchain.
    ///
    /// # Arguments
    /// * `event` - A reference to an `EventDetails` object containing details of the blockchain event.
    ///
    /// # Returns
    /// An `Option<String>` containing relevant information derived from the event, or `None` if no information is extracted.
    async fn process_event(&self, event: &EventDetails<PolkadotConfig>) -> Result<(), Box<dyn Error>>;
}

/// Represents a client for interacting with the Cyborg blockchain.
///
/// This struct is used to interact with the Cyborg blockchain, manage key pairs,
/// and optionally communicate with IPFS or node URIs.
pub struct CyborgClient {
    pub(crate) client: OnlineClient<PolkadotConfig>,
    pub(crate) keypair: Keypair,
    pub ipfs_client: Option<PinataApi>,
    pub node_uri: Option<String>,
    pub identity: (AccountId32, u64),
    pub creator: AccountId32,
}

/// Implementation of the `BlockchainClient` trait for `CyborgClient`.
#[async_trait]
impl BlockchainClient for CyborgClient {
    /// Registers a worker with the Cyborg parachain.
    ///
    /// # Returns
    /// A `Result` indicating success or an error if registration fails.
    async fn register_worker(&self) -> Result<(), Box<dyn Error>> {

        let worker_specs = specs::gather_worker_spec().await?;

        let worker_registration = substrate_interface::api::tx()
            .edge_connect()
            .register_worker(
                worker_specs.domain, 
                worker_specs.latitude, 
                worker_specs.longitude, 
                worker_specs.ram,
                worker_specs.storage, 
                worker_specs.cpu,
            );

        println!("Transaction Details:");
        println!("Module: {:?}", worker_registration.pallet_name());
        println!("Call: {:?}", worker_registration.call_name());
        println!("Parameters: {:?}", worker_registration.call_data());

        let worker_registration_events= self.client
            .tx()
            .sign_and_submit_then_watch_default(&worker_registration, &self.keypair)
            .await
            .map(|e| {
                println!("Worker registration submitted, waiting for transaction to be finalized...");
                e
            })?
            .wait_for_finalized_success()
            .await?;



        let registration_event = 
            worker_registration_events.find_first::<substrate_interface::api::edge_connect::events::WorkerRegistered>()?;
        if let Some(event) = registration_event {

            let worker_file_json = serde_json::to_string(&WorkerData {
                creator: event.creator.clone().to_string(),
                worker: event.worker.clone(),
            })?;

            let package_dir_path = Path::new("/var/lib/cyborg/worker-node/packages");
            let config_dir_path = Path::new("/var/lib/cyborg/worker-node/config");
            let file_path = config_dir_path.join(CONFIG_FILE_NAME);

            if !fs::metadata(&config_dir_path).is_ok() {
                fs::create_dir_all(&config_dir_path)?;
            }

            if !fs::metadata(&package_dir_path).is_ok() {
                fs::create_dir_all(&package_dir_path)?;
            }
        
            // Write content to the file (will overwrite existing content)
            fs::write(&file_path, worker_file_json)?;

            println!("Worker registered successfully: {event:?}");
        } else {
            println!("Worker registration failed");
        }

        Ok(())
    }

    /// Starts a mining session by subscribing to finalized blocks and listening for events.
    ///
    /// # Returns
    /// A `Result` indicating success or an error if starting the session fails.
    async fn start_mining_session(&self) -> Result<(), Box<dyn Error>> {
        println!("Starting mining session...");

        info!("============ event_listener_tester ============");

        let mut blocks = self.client.blocks().subscribe_finalized().await?;

        while let Some(Ok(block)) = blocks.next().await {
            //info!("New block imported: {:?}", block.hash());

            let events = block.events().await?;
            
            for event in events.iter() {
                match event {
                    Ok(ev) => {
                        //println!("Processing Event: {:?}", ev);
                        if let Err(e) = self.process_event(&ev).await {
                            println!("Error processing event: {:?}", e);
                        }    
                    },
                    Err(e) => eprintln!("Error decoding event: {:?}", e),
                }
            }
        }

        Ok(())
    }

    /// Processes an event from the blockchain.
    ///
    /// # Arguments
    /// * `event` - A reference to an `EventDetails` object containing event information.
    ///
    /// # Returns
    /// An `Option<String>` that may contain information derived from the event.
    async fn process_event(
        &self, 
        event: &EventDetails<PolkadotConfig>
    ) -> Result<(), Box<dyn Error>> {
        // subscription_builder.subscribe_to::<cyborg_node::pallet_task_management::events::TaskScheduled>();
        // subscription_builder.subscribe_to::<cyborg_node::pallet_task_management::events::SubmittedCompletedTask>();
        // subscription_builder.subscribe_to::<cyborg_node::pallet_task_management::events::VerifierResolverAssigned>();
        // subscription_builder.subscribe_to::<cyborg_node::pallet_task_management::events::VerifiedCompletedTask>();
        // subscription_builder.subscribe_to::<cyborg_node::pallet_task_management::events::ResolvedCompletedTask>();
        // subscription_builder.subscribe_to::<cyborg_node::pallet_task_management::events::TaskReassigned>();

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
                return Err(Box::new(e));
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
                return Err(Box::new(e));
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
                return Err(Box::new(e));
            }
            _ => {} // Skip non-matching events
        }

        // Check for TaskScheduled event
        match event.as_event::<substrate_interface::api::task_management::events::TaskScheduled>() {
            Ok(Some(task_scheduled)) => {
                let assigned_worker = &task_scheduled.assigned_worker;

                if *assigned_worker == self.identity {
                    if let Some(ipfs_client) = &self.ipfs_client {

                        let ipfs_hash_bounded: Vec<u8> = BoundedVec::encode(&task_scheduled.task);
                        
                        let ipfs_hash = String::from_utf8_lossy(&ipfs_hash_bounded);

                        println!("Ipfs hash: {:?}", ipfs_hash);
                        
                        println!("New task scheduled for worker: {:?}", task_scheduled);

                        let result = download_and_execute_work_package("bafybeic5sgq6obgfg6xine6cf4qpv7xrvnzst5ufyxnzbnzvcafuif56j4/ipfs_test").await;

                        //TODO process zk_files
                        let zk_files = download_and_extract_zk_files("Qmf9v8VbJ6WFGbakeWEXFhUc91V1JG26grakv3dTj8rERh").await;

                        if let Some(Ok(output)) = result {
                            println!("Operation sucessful: {:?}", output);
                            submit_result_onchain(&self.client, &self.keypair, &ipfs_client, output, task_scheduled.task_id).await;
                        } else {
                            println!("result: {:?}", result);
                            println!("Failed to execute command");
                        }
                    } else {
                        return Err("IPFS client not initialized".into());
                    }
                }
            }
            Err(e) => {
                println!("Error decoding WorkerStatusUpdated event: {:?}", e);
                return Err(Box::new(e));
            }
            _ => {} // Skip non-matching events
        }


        Ok(())
    }
}

/*

TODO Implement function for verifying worker registration once edge-connect pallet is updated, might look something like this

pub async fn verify_worker_registration(
    api: &OnlineClient<PolkadotConfig>, 
    worker_data: WorkerData,
    signer: AccountId32
) -> Result<bool, Box<dyn std::error::Error>> {
    let worker_storage_query = cyborg_metadata::storage().edge_connect().worker_clusters(signer.clone(), 0);
    let worker = api
        .storage()
        .at_latest()
        .await?
        .fetch(&worker_storage_query)
        .await?;

    println!("Worker Details: {:?}", worker);

    println!("worker: {:?}", worker);

    // If worker data exists in the storage, decode and verify the domain
    if let Some(_worker) = worker {
        Ok(true)
    } else {
        Ok(false)
    }
}

*/


pub async fn submit_result_onchain(
    api: &OnlineClient<PolkadotConfig>,
    signer_keypair: &Keypair,
    ipfs_client: &PinataApi,
    result: Output,
    task_id: u64,
) {
    dbg!(&result);
    let result_raw_data = String::from_utf8(result.stdout).expect("Invalid UTF-8 output");
    dbg!(&result_raw_data);

    let cid = publish_on_ipfs(result_raw_data.clone(), ipfs_client).await;
    let chain_result = submit_to_chain(api, signer_keypair, cid, task_id, result_raw_data).await;

    match chain_result {
        Ok(_) => {
            println!("Result submitted to chain successfully");
        }
        Err(e) => {
            println!("Failed to submit result to chain: {:?}", e);
        }
    }
}

pub async fn publish_on_ipfs(result: String, ipfs_client: &PinataApi) -> String {
    println!("Publishing on IPFS: {:?}", result);

    let ipfs_res = ipfs_client.pin_json(PinByJson::new(result)).await;
    match ipfs_res {
        Ok(res) => {
            println!("Published on IPFS: {:?}", res);
            res.ipfs_hash
        }
        Err(e) => {
            println!("Failed to publish on IPFS: {:?}", e);
            String::new()
        }
    }
}

pub async fn submit_to_chain(api: &OnlineClient<PolkadotConfig>, signer_keypair: &Keypair, result: String, task_id: u64, task_output: String)
    -> Result<(), Box<dyn std::error::Error>> 
{
    let result_cid: BoundedVec<u8> = BoundedVec::from(BoundedVec(result.as_bytes().to_vec()));

    let completed_hash = H256::from(blake2_256(task_output.as_bytes()));

    let result_submission_tx = substrate_interface::api::tx()
        .task_management()
        .submit_completed_task(
            task_id, 
            completed_hash, 
            result_cid, 
        );

    println!("Transaction Details:");
    println!("Module: {:?}", result_submission_tx.pallet_name());
    println!("Call: {:?}", result_submission_tx.call_name());
    println!("Parameters: {:?}", result_submission_tx.call_data());

    let result_submission_events= api
        .tx()
        .sign_and_submit_then_watch_default(&result_submission_tx, signer_keypair)
        .await
        .map(|e| {
            println!("Result submitted, waiting for transaction to be finalized...");
            e
        })?
        .wait_for_finalized_success()
        .await?;

    let submission_event = 
        result_submission_events.find_first::<substrate_interface::api::task_management::events::SubmittedCompletedTask>()?;
    if let Some(event) = submission_event {
        println!("Task submitted successfully: {event:?}");
    } else {
        println!("Task submission failed");
    }

    Ok(())
}

pub async fn download_and_execute_work_package(
    ipfs_cid: &str,
) -> Option<Result<std::process::Output, std::io::Error>> {
    info!("ipfs_hash: {}", ipfs_cid);
    println!("Starting download of ipfs hash: {}", ipfs_cid);
    info!("============ downloading_file ============");

    // TODO: validate its a valid ipfs hash
    let url = format!("https://ipfs.io/ipfs/{}", ipfs_cid);

    let response = get(&url).await;
    
    match response {
        Ok(response) => {
            if !response.status().is_success() {
                eprintln!("Error: {}", response.status());
                return None; 
            }

            let response_bytes = match response.bytes().await {
                Ok(bytes) => bytes,
                Err(e) => {
                    eprintln!("Failed to read response bytes: {}", e);
                    return None;
                }
            };

            println!("Downloaded {} bytes from Crust's IPFS gateway.", response_bytes.len());

            let dir_path = Path::new("/var/lib/cyborg/worker-node/packages");
            let file_path = dir_path.join(WORK_PACKAGE_DIR);

            if !dir_path.exists() {
                if let Err(e) = fs::create_dir_all(&dir_path) {
                    eprintln!("Failed to create directory: {}", e);
                    return None;
                }
            }

            let mut file = match File::create(&file_path) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("Failed to create file: {}", e);
                    return None;
                }
            };

            if let Err(e) = file.write_all(&response_bytes) {
                eprintln!("Failed to write to file: {}", e);
                return None;
            }

            // File needs to be dropped, else there will be a race condition and the file will not be executable
            drop(file);

            let mut perms = match fs::metadata(&file_path) {
                Ok(meta) => meta.permissions(),
                Err(e) => {
                    eprintln!("Failed to retrieve file metadata: {}", e);
                    return None;
                }
            };
            perms.set_mode(perms.mode() | 0o111);

            if let Err(e) = fs::set_permissions(&file_path, perms) {
                eprintln!("Failed to set file permissions: {}", e);
                return None;
            }

            match Command::new(&file_path).stdout(Stdio::piped()).spawn() {
                Ok(child_process) => Some(Ok(child_process.wait_with_output().ok()?)),
                Err(e) => {
                    eprintln!("Failed to execute command: {}", e);
                    None 
            }
        }
        }
        Err(e) => {
            println!("Error: {}", e);
            return None;
        }
    }
}

async fn download_and_extract_zk_files(ipfs_cid: &str) -> Option<ZkFiles> {
    let url = format!("https://ipfs.io/ipfs/{}", ipfs_cid);

    let response = get(&url).await.unwrap();

    if !response.status().is_success() {
        return None;
    }

    let bytes = response.bytes().await.unwrap();

    let reader = std::io::Cursor::new(bytes);

    let mut archive = ZipArchive::new(reader).unwrap();

    let mut unpacked_files = ZkFiles {
        zk_public_input: None,
        zk_circuit: None,
    };

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();

        let filename = file.name().to_string();

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();

        match filename.as_str() {
            "zk_public_input.json" => unpacked_files.zk_public_input = Some(String::from_utf8(buffer).unwrap()),
            "zk_circuit.circom" => unpacked_files.zk_circuit = Some(String::from_utf8(buffer).unwrap()),
            _ => {
                println!("Unexpected file in zip: {}", filename);
            }
        }
    }

    Some(unpacked_files)
}

/// Can send stages 1-4 of the zk-verification process to the cyborg-agent, which will send it to the frontend
async fn emit_zk_stage_signal(stage: u8) -> Result<(), Box<dyn Error>> {
    let connection = Connection::system().await?;

    let msg = Message::signal(
        "/com/cyborg/CyborgAgent",
        "com.cyborg.AgentZkInterface",
        "ZkStageChanged",
    )?
    .build(&stage).unwrap();

    connection.send(&msg).await?;

    Ok(())
}
/*

fn worker_retain_after_restart(reg_event: EventWorkerRegistered) -> Option<WorkerData> {
    let registered_worker_data = WorkerData {
        creator: reg_event.creator.to_ss58check(),
        worker: reg_event.worker,
        domain: String::from_utf8_lossy(reg_event.domain.as_bytes_ref()).to_string(),
        domain_encoded: reg_event.domain.into(),
    };

    let registered_worker_json = serde_json::to_string_pretty(&registered_worker_data);
    info!("{:?}", &registered_worker_json);

    use std::{fs::File, path::Path};

    let config_path = Path::new(CONFIG_FILE_NAME);
    match File::create(config_path) {
        Err(e) => {
            error!("{}", e);
            None
        }
        Ok(mut created_file) => {
            created_file
                .write_all(registered_worker_json.unwrap().as_bytes())
                .unwrap_or_else(|_| panic!("Unable to write file : {:?}", config_path.to_str()));
            info!(
                "✅✅Saved worker registration data to file: {:?}✅✅ ",
                config_path.to_str()
            );
            Some(registered_worker_data)
        }
    }
}
    */