use codec::{Decode, Encode, EncodeLike};
use cyborg_runtime::apis::{TaskManagementEventsApi, VerifyWorkerRegistration};
use ipfs_api_backend_hyper::TryFromUri;
use ipfs_api_backend_hyper::{IpfsApi, IpfsClient};
use log::info;
//use sc_client_api::BlockchainEvents;
use serde::{Deserialize, Serialize};
//use sp_api::ProvideRuntimeApi;
//use sp_blockchain::HeaderBackend;
use sp_core::{sr25519, ConstU32, Pair};
//use sp_runtime::traits::Block;
use sp_runtime::BoundedVec;
use std::{env, fs, option, sync::Arc};
use substrate_api_client::ac_primitives::{
    AssetRuntimeConfig, GenericExtrinsicParams, PlainTip, WithExtrinsicParams,
};
use substrate_api_client::GetStorage;
use substrate_api_client::{rpc::TungsteniteRpcClient, Api};

use url::Url;

pub const CONFIG_FILE_NAME: &str = "registered_worker_config.json";

pub const IPFS_DEFAULT_URI: &str =
	"https://8be9886d720942e0be9c10bc4351e9dd:ea84a88bd688458188735bff8c576e90@ipfs.infura.io:5001/api/v0";

pub type SubstrateClientApi = Api<
    WithExtrinsicParams<
        AssetRuntimeConfig,
        GenericExtrinsicParams<AssetRuntimeConfig, PlainTip<u128>>,
    >,
    TungsteniteRpcClient,
>;

// datastructure for worker registartion persistence
#[derive(Debug, Clone, PartialEq, Eq, Decode, Encode, Serialize, Deserialize)]
pub struct WorkerData {
    pub creator: String,
    pub worker: (sr25519::Public, u64),
    pub domain: String,
    pub domain_encoded: Vec<u8>,
}

pub struct WorkerConfig {
    domain: BoundedVec<u8, ConstU32<128>>,
    latitude: i32,
    longitude: i32,
    ram: u64,
    storage: u64,
    cpu: u16,
}

pub async fn start_worker() {
    info!("worker_starting");

    // export CYBORG_WORKER_KEY="e5be9a5092b81bca64be81d212e7f2f9eba183bb7a90954f7b76361f6edb5c0a" # //Alice
    // export CYBORG_WORKER_DOMAIN="example.com" # replace with your domain

    // IPFS="https://<infura_project_api_key>:<infura_api_secret_key>@ipfs.infura.io:5001/api/v0"
    // run zombienet `zombienet --provider native spawn ./zombienet.toml`

    let worker_key = env::var("CYBORG_WORKER_KEY").expect("CYBORG_WORKER_KEY not set");
    let worker_domain = env::var("CYBORG_WORKER_DOMAIN").expect("CYBORG_WORKER_DOMAIN not set");
    let ipfs_credentials = env::var("IPFS");

    let ipfs_url = ipfs_credentials
        .ok()
        .and_then(|creds| creds.parse::<Url>().ok())
        .unwrap_or_else(|| {
            IPFS_DEFAULT_URI
                .parse::<Url>()
                .expect("Invalid default IPFS URI")
        });

    let mut key_32 = [0u8; 32];

    key_32[..].copy_from_slice(&hex::decode(worker_key).unwrap());

    let key = sr25519::Pair::from_seed(&key_32);

    // WARNING: only works on zombienet because of port
    // TODO: get the port from cli arg
    let api_client = TungsteniteRpcClient::new_with_port("ws://127.0.0.1", 9988, 2).unwrap();
    //let mut api = Api::<DefaultRuntimeConfig, _>::new(api_client).unwrap();

    let mut api = Api::<
        WithExtrinsicParams<
            AssetRuntimeConfig,
            GenericExtrinsicParams<AssetRuntimeConfig, PlainTip<u128>>,
        >,
        TungsteniteRpcClient,
    >::new(api_client)
    .unwrap();

    api.set_signer(key.clone().into());

    let ipfs_client = IpfsClient::build_with_base_uri(ipfs_url.to_string().parse().unwrap())
        .with_credentials(ipfs_url.username(), ipfs_url.password().unwrap());

    let version_out = ipfs_client.version().await;
    info!("version_out: {:?}", &version_out);

    let worker_config = gather_worker_spec(worker_domain);
    let worker_data = bootstrap_worker(api, worker_config).await.unwrap();
    // TO CHANGE:  custom_event_listener::event_listener_tester(client, api, ipfs_client, worker_data).await;
}

pub async fn bootstrap_worker(
    api: SubstrateClientApi,
    worker_config: WorkerConfig,
) -> option::Option<WorkerData> {
    match fs::read_to_string(CONFIG_FILE_NAME) {
        Err(_e) => {
            info!("worker registation not found, registering worker");
            register_worker_on_chain(api, worker_config).await
        }
        Ok(data) => {
            let worker_data: WorkerData = serde_json::from_str(&data).unwrap();
            if verify_worker_registration(&api, worker_data.clone()).await {
                Some(worker_data)
            } else {
                register_worker_on_chain(api, worker_config).await
            }
        }
    }
}

pub async fn verify_worker_registration(api: &SubstrateClientApi, worker_data: WorkerData) -> bool {
    let storage_key: Option<Vec<u8>> = api
        .get_storage_map(
            "pallet_edge_connect",        // Pallet name
            "WorkerClusters",             // Storage map name
            &worker_data.worker.encode(), // Encoded (AccountId, WorkerId) as the storage key
            None,                         // Query the latest state
        )
        .unwrap_or(None); // Handle potential error

    // If worker data exists in the storage, decode and verify the domain
    if let Some(encoded_worker) = storage_key {
        let stored_worker_data: WorkerData =
            Decode::decode(&mut &encoded_worker[..]).expect("Failed to decode worker data");

        stored_worker_data.domain_encoded == worker_data.domain_encoded
    } else {
        false
    }

    // client
    //     .runtime_api()
    //     .verify_worker_registration(
    //         client.info().best_hash,
    //         (worker_data.worker.0.into(), worker_data.worker.1.into()),
    //         worker_data.domain_encoded.try_into().unwrap_or_default(),
    //     )
    //     .unwrap_or(false)
}

use sys_info;

pub fn gather_worker_spec(domain: String) -> WorkerConfig {
    let domain = BoundedVec::try_from(domain.as_bytes().to_vec()).unwrap();
    let ram = sys_info::mem_info().unwrap().total;
    let storage = sys_info::disk_info().unwrap().total;
    let cpu = sys_info::cpu_num().unwrap().try_into().unwrap();
    WorkerConfig {
        domain,
        latitude: 0,
        longitude: 0,
        ram,
        storage,
        cpu,
    }
}

use cyborg_runtime as runtime;
use log::error;
use sp_core::H256;
use std::process::{Child, ChildStdout};
use substrate_api_client::{SubmitAndWatch, XtStatus};

pub async fn submit_result_onchain(
    api: &SubstrateClientApi,
    ipfs_client: &IpfsClient,
    mut result: Child,
    task_id: u64,
) {
    dbg!(&result);
    let result_raw_data = result.stdout.take().unwrap();
    dbg!(&result_raw_data);

    let hash = publish_on_ipfs(result_raw_data, ipfs_client).await;
    submit_to_chain(api, hash, task_id).await;
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

pub async fn submit_to_chain(api: &SubstrateClientApi, result: String, task_id: u64) {
    let result = BoundedVec::try_from(result.as_bytes().to_vec()).unwrap();

    let completed_hash = H256::random();

    let register_call = runtime::RuntimeCall::TaskManagement(
        runtime::pallet_task_management::Call::submit_completed_task {
            task_id,
            completed_hash,
            result,
        },
    );

    dbg!(&register_call);

    let tr_tx = api.compose_extrinsic_offline(register_call, api.get_nonce().unwrap());
    info!("{:?}", &tr_tx);

    let ext_response = api.submit_and_watch_extrinsic_until(tr_tx, XtStatus::InBlock);
    if let Ok(ext_response) = ext_response {
        info!("Task submited successfully ✅✅✅✅✅✅✅✅✅✅✅✅✅");
        info!("{:?}", ext_response);
    } else {
        error!("❌ Something went wrong on result submission ❌");

        info!("{:?}", &ext_response);
    }
}

use futures::StreamExt;
use sc_client_api::BlockchainEvents;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_core::hexdisplay::AsBytesRef;
use sp_runtime::traits::Block;

pub async fn event_listener_tester<T, U>(
    client: Arc<T>,
    api: SubstrateClientApi,
    ipfs_client: IpfsClient,
    worker_data: WorkerData,
) where
    U: Block,
    T: ProvideRuntimeApi<U> + HeaderBackend<U> + BlockchainEvents<U>,
    T::Api: TaskManagementEventsApi<U>,
{
    info!("============ event_listener_tester ============");

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

use sp_core::crypto::Ss58Codec;

use substrate_api_client::ac_node_api::StaticEvent;

#[derive(Debug, Clone, PartialEq, Eq, Decode)]
struct EventWorkerRegistered {
    creator: sr25519::Public,
    worker: (sr25519::Public, u64),
    domain: BoundedVec<u8, ConstU32<128>>,
}

impl StaticEvent for EventWorkerRegistered {
    const PALLET: &'static str = "EdgeConnect";
    const EVENT: &'static str = "WorkerRegistered";
}

pub async fn register_worker_on_chain(
    api: SubstrateClientApi,
    worker_config: WorkerConfig,
) -> Option<WorkerData> {
    // let domain = BoundedVec::try_from(worker_domain.as_bytes().to_vec()).unwrap();

    let WorkerConfig {
        domain,
        latitude,
        longitude,
        ram,
        storage,
        cpu,
    } = worker_config;

    let register_call =
        runtime::RuntimeCall::EdgeConnect(runtime::pallet_edge_connect::Call::register_worker {
            domain,
            latitude,
            longitude,
            ram,
            storage,
            cpu,
        });

    let tr_tx = api.compose_extrinsic_offline(register_call, api.get_nonce().unwrap());
    info!("{:?}", &tr_tx);

    let ext_response = api.submit_and_watch_extrinsic_until(tr_tx, XtStatus::InBlock);
    info!("{:?}", &ext_response);

    match ext_response {
        Ok(ext_response_succ) => {
            info!("{:?}", &ext_response_succ);
            info!("Worker registered successfully ✅✅✅✅✅✅✅✅✅✅✅✅✅");
            info!("Events: {:?}", ext_response_succ.events);

            for event in ext_response_succ.events.unwrap() {
                if event.pallet_name() == "EdgeConnect" {
                    let decoded_event = event.as_event::<EventWorkerRegistered>();
                    info!("{:?}", &decoded_event);
                    match decoded_event {
                        Err(_) | Ok(None) => {
                            error!("❌Event not decoded properly❌");
                        }
                        Ok(Some(registration_event)) => {
                            return worker_retain_after_restart(registration_event);
                        }
                    }
                }
            }
            None
        }
        Err(e) => {
            error!(
                "Somethign went wrong while registering worker ❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌"
            );
            error!("{:?}", e);
            error!("RESTART The worker node with proper environment variables");
            None
        }
    }
}

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
