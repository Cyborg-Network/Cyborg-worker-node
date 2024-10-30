use codec::{Decode, Encode, EncodeLike};
use ipfs_api_backend_hyper::TryFromUri;
use ipfs_api_backend_hyper::{IpfsApi, IpfsClient};
use log::info;
//use sc_client_api::BlockchainEvents;
use serde::{Deserialize, Serialize};
//use sp_api::ProvideRuntimeApi;
//use sp_blockchain::HeaderBackend;
//use sp_runtime::traits::Block;
use subxt::utils::AccountId32;
use std::{env, fs, option, sync::Arc};
use subxt::{OnlineClient, PolkadotConfig, SubstrateConfig};
use cyborg_metadata::runtime_types::bounded_collections::bounded_vec::BoundedVec;
use subxt::tx::{signer, TxStatus};
use subxt_signer::{SecretUri, sr25519::Keypair};
use std::str::FromStr;

use url::Url;

use crate::cyborg_metadata;
use crate::cyborg_metadata::runtime_types::cyborg_primitives::worker;
use crate::substrate_api::substrate_interface;

pub const CONFIG_FILE_NAME: &str = "registered_worker_config.json";

const PRIVATE_KEY: &str = "CYBORG_WORKER_KEY";

pub const IPFS_DEFAULT_URI: &str =
	"https://8be9886d720942e0be9c10bc4351e9dd:ea84a88bd688458188735bff8c576e90@ipfs.infura.io:5001/api/v0";

// datastructure for worker registartion persistence
#[derive(Debug, Clone, PartialEq, Eq, Decode, Encode, Serialize, Deserialize)]
pub struct WorkerData {
    pub creator: String,
    pub worker: (AccountId32, u64),
    pub domain: String,
    pub domain_encoded: Vec<u8>,
}

pub struct WorkerConfig {
    domain: BoundedVec<u8>,
    latitude: i32,
    longitude: i32,
    ram: u64,
    storage: u64,
    cpu: u16,
}

#[derive(Deserialize)]
struct IpResponse {
    ip: String,
}

pub async fn start_worker() -> Result<(), Box<dyn std::error::Error>> {
    info!("worker_starting");

    // export CYBORG_WORKER_KEY="e5be9a5092b81bca64be81d212e7f2f9eba183bb7a90954f7b76361f6edb5c0a" # //Alice
    // export CYBORG_WORKER_DOMAIN="example.com" # replace with your domain

    // IPFS="https://<infura_project_api_key>:<infura_api_secret_key>@ipfs.infura.io:5001/api/v0"
    // run zombienet `zombienet --provider native spawn ./zombienet.toml`

    let uri = SecretUri::from_str("bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice")
        .expect("valid URI");
    let signer_keypair = Keypair::from_uri(&uri)
        .expect("valid keypair");

    let worker_key = signer_keypair;
    let ipfs_credentials = env::var("IPFS");

    let ipfs_url = ipfs_credentials
        .ok()
        .and_then(|creds| creds.parse::<Url>().ok())
        .unwrap_or_else(|| {
            IPFS_DEFAULT_URI
                .parse::<Url>()
                .expect("Invalid default IPFS URI")
        });

    // WARNING: only works on zombienet because of port
    // TODO: Make url somewhat dynamic
    // PolkadotConfig is used for parachains
    let api = OnlineClient::<PolkadotConfig>::from_url("ws://localhost:9988").await?;

    let ipfs_client = IpfsClient::build_with_base_uri(ipfs_url.to_string().parse().unwrap())
        .with_credentials(ipfs_url.username(), ipfs_url.password().unwrap());

    let version_out = ipfs_client.version().await;
    info!("version_out: {:?}", &version_out);

    let worker_config = gather_worker_spec().await?;
    let _worker_data = bootstrap_worker(api, worker_config, worker_key).await;
    // TO CHANGE:  custom_event_listener::event_listener_tester(client, api, ipfs_client, worker_data).await;
    Ok(())
}

pub async fn gather_worker_spec() -> Result<WorkerConfig, Box<dyn std::error::Error>> {

    let response = reqwest::get("https://api.ipify.org?format=json").await?
        .json::<IpResponse>().await?;

    Ok(WorkerConfig {
        domain: BoundedVec::from(BoundedVec(response.ip.as_bytes().to_vec())),
        latitude: 0,
        longitude: 0,
        ram: 0,
        storage: 0,
        cpu: 0,
    })
}

pub async fn bootstrap_worker(
    api: OnlineClient<PolkadotConfig>,
    worker_config: WorkerConfig,
    signer_keypair: Keypair
) -> Result<(), Box<dyn std::error::Error>> {
    match fs::read_to_string(CONFIG_FILE_NAME) {
        Err(_e) => {
            println!("Worker registation not found, registering worker");
            register_worker_on_chain(api, worker_config, signer_keypair.clone()).await?;
            Ok(())
        }
        Ok(_data) => {
            /*
            let worker_data: WorkerData = serde_json::from_str(&data).unwrap();
            if verify_worker_registration(&api, worker_data.clone()).await {
                Some(worker_data)
            } else {
                register_worker_on_chain(api, worker_config, signer_keypair).await
            }
            */
            // TODO - verify worker registration as soon as verify_worker_registration is implemented
            println!("Worker registration found, skipping worker registration");
            Ok(())
        }
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

use log::error;
use subxt::utils::H256;
use std::process::{Child, ChildStdout};

pub async fn submit_result_onchain(
    api: &OnlineClient<PolkadotConfig>,
    signer_keypair: Keypair,
    ipfs_client: &IpfsClient,
    mut result: Child,
    task_id: u64,
) {
    dbg!(&result);
    let result_raw_data = result.stdout.take().unwrap();
    dbg!(&result_raw_data);

    let hash = publish_on_ipfs(result_raw_data, ipfs_client).await;
    submit_to_chain(api, signer_keypair, hash, task_id).await;
}

pub async fn publish_on_ipfs(result: ChildStdout, ipfs_client: &IpfsClient) -> String {
    let ipfs_res = ipfs_client.add(result).await;
    if let Ok(ipfs_res) = ipfs_res {
        let hash = ipfs_res.hash;
        dbg!(&hash);
        hash
    } else {
        error!("Failed to publish on IPFS");
        String::new()
    }
}

pub async fn submit_to_chain(api: &OnlineClient<PolkadotConfig>, signer_keypair: Keypair, result: String, task_id: u64)
    -> Result<(), Box<dyn std::error::Error>> 
{
    let result: BoundedVec<u8> = BoundedVec::from(BoundedVec(result.as_bytes().to_vec()));

    let completed_hash = H256::random();

    let result_submission_tx = cyborg_metadata::tx()
        .task_management()
        .submit_completed_task(
            task_id, 
            completed_hash, 
            result, 
        );

    let result_submission_events= api
        .tx()
        .sign_and_submit_then_watch_default(&result_submission_tx, &signer_keypair)
        .await
        .map(|e| {
            println!("Result submission submitted, waiting for transaction to be finalized...");
            e
        })?
        .wait_for_finalized_success()
        .await?;

    let submission_event = 
        result_submission_events.find_first::<cyborg_metadata::task_management::events::SubmittedCompletedTask>()?;
    if let Some(event) = submission_event {
        println!("Task submitted successfully: {event:?}");
    } else {
        println!("Task submission failed");
    }

    Ok(())
}

//use futures::StreamExt;
//use sc_client_api::BlockchainEvents;
//use sp_api::ProvideRuntimeApi;
//use sp_blockchain::HeaderBackend;
//use sp_core::hexdisplay::AsBytesRef;
//use sp_runtime::traits::Block;

pub async fn event_listener_tester<T, U>(
    client: Arc<T>,
    api: OnlineClient<PolkadotConfig>,
    ipfs_client: IpfsClient,
    worker_data: WorkerData,
) //where
   // U: Block,
    //T: ProvideRuntimeApi<U> + HeaderBackend<U> + BlockchainEvents<U>,
    //T::Api: TaskManagementEventsApi<U>,
-> Result <(), Box<dyn std::error::Error>> {
    info!("============ event_listener_tester ============");

    let mut blocks = api.blocks().subscribe_finalized().await?;
    
    while let Some(Ok(block)) = blocks.next().await {
        info!("New block imported: {:?}", block.hash());

        let events = block.events().await?;
        
        for event in events.iter() {
            match event {
                Ok(ev) => {
                    println!("Event: {:?}", ev);
                    
                    // Optionally, match specific events by type
                    if let Some(scheduling_event) = ev.as_event::<cyborg_metadata::task_management::events::TaskScheduled>()? {
                        println!("Task Scheduled: {:?}", scheduling_event);
                    }
                },
                Err(e) => eprintln!("Error decoding event: {:?}", e),
            }
        }
    }

    Ok(())

    /*

    let mut blocks = client.every_import_notification_stream();

    while let Some(block_import_notification) = blocks.next().await {
        info!("block_import_notification");

        let block_hash = block_import_notification.hash;

        match client.runtime_api().get_recent_events(block_hash) {
            Ok(event_vec) => {
                info!("{:?}", &event_vec);
                for event in event_vec {
                    if let cyborg_runtime::pallet_task_management::Event::TaskScheduled {
                        assigned_worker,
                        task_owner,
                        task_id,
                        task,
                        ..
                    } = event
                    {
                        if assigned_worker == (worker_data.worker.0.into(), worker_data.worker.1) {
                            info!("task_id: {}, task_owner: {}", task_id, task_owner);
                            let ipfs_hash = String::from_utf8_lossy(task.as_bytes_ref());
                            info!("{}", &ipfs_hash);

                            let result =
                                download_and_execute_work_package(ipfs_hash.as_ref(), &ipfs_client)
                                    .await;
                            if let Some(Ok(output)) = result {
                                info!("{:?}", &output);
                                submit_result_onchain(&api, &ipfs_client, output, task_id).await;
                            } else {
                                info!("result: {:?}", result);
                                error!("Failed to execute command");
                            }
                        }
                    }
                }
            }
            Err(e) => {
                error!("{}", e);
            }
        }
    }

    */
}

use futures::TryStreamExt;
use ipfs_api_backend_hyper::{self};
use std::fs::File;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::{Command, Stdio};

pub const WORK_PACKAGE_DIR: &str = "work_package_binary";

pub async fn download_and_execute_work_package(
    ipfs_hash: &str,
    ipfs_client: &IpfsClient,
) -> Option<Result<std::process::Child, std::io::Error>> {
    info!("ipfs_hash: {}", ipfs_hash);
    info!("============ download_file ============");
    // TODO: validate its a valid ipfs hash
    match ipfs_client
        .cat(&format!("/ipfs/{ipfs_hash}"))
        .map_ok(|chunk| chunk.to_vec())
        .try_concat()
        .await
    {
        Err(e) => {
            error!("{}", e);
            None
        }
        Ok(data) => {
            info!("got data from ipfs with length of {}", &data.len());
            let w_package_path = Path::new(WORK_PACKAGE_DIR);
            if !w_package_path.exists() {
                fs::create_dir(w_package_path).unwrap();
            }
            let file_path = format!("./{WORK_PACKAGE_DIR}/{ipfs_hash}");

            let mut file = File::create(&file_path).unwrap();
            let mut perms = file.metadata().unwrap().permissions();
            perms.set_mode(perms.mode() | 0o111);

            file.write_all(&data).unwrap();

            file.set_permissions(perms).unwrap();
            Some(Command::new(file_path).stdout(Stdio::piped()).spawn())
        }
    }
}

pub async fn register_worker_on_chain(
    api: OnlineClient<PolkadotConfig>,
    worker_config: WorkerConfig,
    signer_keypair: Keypair
) -> Result<(), Box<dyn std::error::Error>> {
    // TODO add call to agent to get worker specs

    let worker_registration = cyborg_metadata::tx()
        .edge_connect()
        .register_worker(
            worker_config.domain, 
            0, 
            0, 
            0,
            0, 
            0
        );

    let worker_registration_events= api
        .tx()
        .sign_and_submit_then_watch_default(&worker_registration, &signer_keypair)
        .await
        .map(|e| {
            println!("Worker registration submitted, waiting for transaction to be finalized...");
            e
        })?
        .wait_for_finalized_success()
        .await?;

    let registration_event = 
        worker_registration_events.find_first::<cyborg_metadata::edge_connect::events::WorkerRegistered>()?;
    if let Some(event) = registration_event {
        println!("Worker registered successfully: {event:?}");
    } else {
        println!("Worker registration failed");
    }

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