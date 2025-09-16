use bollard::{query_parameters::{ListContainersOptions, RemoveContainerOptions}, Docker};
use serde::Serialize;
use subxt::utils::AccountId32;
use tokio::{sync::RwLock, task::JoinHandle};
use std::{fs, sync::Arc};
use crate::{
    config::{
        self, get_parachain_client, get_paths, CONTAINER_PREFIX, CURRENT_TASK_PATH
    }, error::{Error, Result}, log, parachain_interactor::identity::update_identity_file, parent_runtime::inference::CURRENT_SERVER, substrate_interface::api::{
        runtime_types::cyborg_primitives::task::TaskStatusType, 
        task_management::events::task_scheduled::TaskId
    }, traits::InferenceServer, types::{
        CurrentTask, 
        Miner, 
        ParentRuntime
    }, utils::{substrate_queries::{
        get_currently_assigned_task_id, get_miner_id_assigned_to_task, get_task
    }, tx_builder::{confirm_miner_vacation, pub_confirm_task_reception}, tx_queue::TxOutput}
};

#[derive(Serialize)]
struct TaskOwner {
    address: AccountId32,
}

pub enum TaskPickupReturnType {
    Success(JoinHandle<()>),
    Failure(())
}

/// Write the current task id to the drive
fn update_current_task_file(task_id: TaskId) -> Result<()> {
    println!("Updating current task file...");

    let path = &*CURRENT_TASK_PATH;
    let task_string = serde_json::to_string(&task_id)?;

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(path, task_string)?;

    Ok(())
}

/// Set the task that the miner is currently executing
pub async fn set_current_task(miner: &mut Miner, task: CurrentTask) -> Result<(TaskId, JoinHandle<()>)> {
    println!("Setting current task...");

    let task_owner_string = serde_json::to_string(&TaskOwner{
        address: task.task_owner.clone(),
    })?;

    let task_owner_path = &get_paths()?.task_owner_path;

    update_identity_file(
        task_owner_path,
        &task_owner_string,
    )?;

    update_current_task_file(task.id)?;

    miner.current_task = Some(task.clone());

    let task_id = task.id;
    let handle = handle_spawn_inference_server(
        miner.parent_runtime.clone(), 
        task,
    ).await;

    Ok((task_id, handle))
}

/// Pick up the task that is currently assigned to the miner by accessing the global state and reading the task from the drive
pub async fn pick_up_task(miner: &mut Miner) -> Result<TaskPickupReturnType> {
    println!("Trying to pick up task...");

    let api = get_parachain_client()?;

    let miner_id = &miner
        .miner_identity
        .as_ref()
        .ok_or(Error::identity_not_initialized())?;

    let task_id = match get_currently_assigned_task_id(api, miner_id).await {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Reading taskid from parachain failed: {}", e);

            if let Err(nuke_err) = nuke_all_running_task_containers().await {
                eprintln!("Nuking containers also failed: {}", nuke_err);
                eprintln!("Original error was: {}", e);
            }

            return Err(e.into());
        }
    };

    let task = match get_task(api, task_id).await {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Reading task from parachain failed, nuking all possible remaining running task containers: {}", e);

            if let Err(nuke_err) = nuke_all_running_task_containers().await {
                eprintln!("Nuking containers also failed: {}", nuke_err);
                eprintln!("Original error was: {}", e);
            }

            return Err(e.into());
        }
    };

    let task_miner_id = match get_miner_id_assigned_to_task(api, task_id).await {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Reading assigned miner from parachain failed, nuking all possible remaining running task containerss: {}", e);

            if let Err(nuke_err) = nuke_all_running_task_containers().await {
                eprintln!("Nuking containers also failed: {}", nuke_err);
                eprintln!("Original error was: {}", e);
            }

            return Err(e.into());
        }
    };

    if let Some(identity) = &miner.miner_identity {
        if &task_miner_id != identity {
            nuke_all_running_task_containers().await?;
            return Err("Miner is not assigned to any task, nuking all possible remaining running task containers".into());
        } else {
            match task.task_status {
                // Task is not active anymore, but the miner vacation has not yet been confirmed, for the parachain to assign new tasks we need to confirm
                TaskStatusType::Stopped => {
                    println!("Task already stopped, cleaning up and vacating miner...");

                    let task = CurrentTask {
                        task_type: task.task_kind,
                        task_owner: task.task_owner,
                        id: task_id,
                        container_name: return_task_container_name(task_id)
                    };

                    // This is required because task vacation will require the miner to actually hold a task
                    miner.current_task = Some(task);

                    clean_up_current_task_and_vacate(miner).await?;

                    Ok(TaskPickupReturnType::Failure(()))
                }
                // Task is not active anymore
                TaskStatusType::Vacated => {
                    println!("Miner already vacated, cleaning up...");

                    nuke_all_running_task_containers().await?;

                    Ok(TaskPickupReturnType::Failure(()))
                }
                // Task is assinged but not yet confirmed by the miner, which is necessary so that the parachain won't suspend the miner
                TaskStatusType::Assigned => {
                    println!("Assigned task found, confirming task reception...");

                    let task = CurrentTask {
                        task_type: task.task_kind,
                        task_owner: task.task_owner,
                        id: task_id,
                        container_name: return_task_container_name(task_id)
                    };
                    let (_, handle) = set_current_task(miner, task).await?;

                    let keypair = miner.keypair.clone();
                    tokio::spawn(async move {
                        if let Err(e) = pub_confirm_task_reception(keypair, &task_id).await {
                            println!("Critical error encountered, please contact the support: {}", e);
                        }
                    });

                    Ok(TaskPickupReturnType::Success(handle))
                }
                // Task should already be running and only needs to be picked back up
                TaskStatusType::Running => {
                    let task = CurrentTask {
                        task_type: task.task_kind,
                        task_owner: task.task_owner,
                        id: task_id,
                        container_name: return_task_container_name(task_id)
                    };
                    let (_, handle) = set_current_task(miner, task).await?;

                    Ok(TaskPickupReturnType::Success(handle))
                }
            }
        }
    } else {
        nuke_all_running_task_containers().await?;
        Err("Miner does not have an identity, nuking all possible remaining running task containers".into())
    }
}

/// Nuke all docker containers that are associated with task execution - this is used for radical cleanup, either after task execution or after miner restart
pub async fn nuke_all_running_task_containers() -> Result<()> {
    let docker = Docker::connect_with_local_defaults()?;

    let containers = docker
        .list_containers(Some(ListContainersOptions {
            all: true,
            ..Default::default()
        }))
        .await?;

    if containers.is_empty() {
        println!("No running containers to kill, skipping.")
    }

    for container in containers {
        if let Some(names) = container.names {
            for name in names {
                if name.trim_start_matches('/').starts_with(CONTAINER_PREFIX) {
                    println!("Removing container: {}", name);

                    if let Some(id) = container.id.as_ref() {
                        docker
                            .remove_container(
                                id,
                                Some(RemoveContainerOptions {
                                    force: true,
                                    ..Default::default()
                                }),
                            )
                            .await?;
                    }
                }
            }
        }
    }

    Ok(())
}

/// Conditionally create a container for inference or just spawn the inference server if the container already exists
pub async fn handle_spawn_inference_server(
    parent_runtime: Arc<RwLock<ParentRuntime>>,
    current_task: CurrentTask
) -> JoinHandle<()> {
    tokio::spawn(async move {
        if !check_if_task_container_exists(&current_task.container_name).await.unwrap_or(false) {
            println!("No container exists for the current task, creating...");

            if let Err(e) = parent_runtime
                .read()
                .await
                .process_task(current_task.clone().task_type)
                .await
            {
                println!("Error downloading model archive: {}", e);
            }
        }

        if let Err(e) = parent_runtime
            .read()
            .await
            .spawn_inference_server(&current_task)
            .await
        {
            println!("Error performing inference: {}", e);
        }
    })
}

/// Check if a container exists for the current task
async fn check_if_task_container_exists(task_container_name: &str) -> Result<bool> {
    let docker = Docker::connect_with_local_defaults()?;

    let containers = docker
        .list_containers(Some(ListContainersOptions {
            all: true,
            ..Default::default()
        }))
        .await?;

    for container in containers {
        if let Some(names) = container.names {
            if names.iter().any(|n| n.trim_start_matches('/') == task_container_name) {
                return Ok(true);
            }
        }
    }

    Ok(false) 
}

/// Return the container name for the current task (utility that makes sure that the name is always absolutely the same)
pub fn return_task_container_name(task_id: TaskId) -> String {
    format!("{}{}", CONTAINER_PREFIX, task_id)
}

/// Clean up the task that the miner is currently executing
pub async fn clean_up_current_task_and_vacate(miner: &mut Miner, ) -> Result<()> {
    nuke_all_running_task_containers().await?;

    let keypair = miner.keypair.clone();
    let current_task_id = miner.current_task.as_ref().map(|t| t.id);

    tokio::spawn(async move {
        if let Err(e) = async {
            let tx_queue = config::get_tx_queue()?;
            let current_task_id = current_task_id.ok_or("Miner doesn't hold a current task")?;

            let rx = tx_queue.enqueue(move || {
                let keypair = keypair.clone();
                async move {
                    let _ = confirm_miner_vacation(keypair, current_task_id).await?;
                    Ok(TxOutput::Success)
                }
            }).await?;

            match rx.await {
                Ok(Ok(TxOutput::Success)) => println!("Miner vacation confirmed!"),
                Ok(Err(e)) => println!("Error confirming miner vacation: {}", e),
                _ => println!("Unexpected response for miner vacation confirmation"),
            }

            Ok::<_, Error>(())
        }.await {
            println!("Error confirming miner vacation: {}", e);
        }
    });

    let task_dir = &config::PATHS
        .get()
        .ok_or(Error::config_paths_not_initialized())?
        .task_dir_path;

    // Remove task owner file
    let task_owner_path = &get_paths()?.task_owner_path;
    fs::remove_file(&task_owner_path)?;
    // Remove current task id file
    fs::remove_file(&*CURRENT_TASK_PATH)?;
    // Remove end-user log file
    log::reset_log_file()?;

    miner.current_task = None;

    let server_control = CURRENT_SERVER
        .lock()
        .await
        .take()
        .ok_or(Error::Custom("There is no inference server initialized in CURRENT_SERVER!".to_string()))?;

    server_control.shutdown(task_dir).await?;

    Ok(())
}