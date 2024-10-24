use std::sync::Arc;

use cyborg_runtime::apis::TaskManagementEventsApi;
use futures::StreamExt;
use ipfs_api_backend_hyper::IpfsClient;

use log::{error, info};
use sc_client_api::BlockchainEvents;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_core::hexdisplay::AsBytesRef;
use sp_runtime::traits::Block;

use crate::worker::{
    donwloade_and_execute_tasks::download_and_execute_work_package,
    submit_results::submit_result_onchain,
};

use super::{SubstrateClientApi, WorkerData};

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
