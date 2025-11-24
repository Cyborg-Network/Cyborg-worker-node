<<<<<<< HEAD
use async_trait::async_trait;
use container_manager::{ContainerManager, ContainerProvisionArgs};
use native_manager::{NativeManager, ActiveUser};
=======
use bollard::query_parameters::{
    InspectContainerOptions, RemoveContainerOptions, StartContainerOptions,
};
use bollard::secret::ContainerStateStatusEnum;
use bollard::Docker;
>>>>>>> feature/agent-lib

mod container_manager;
mod native_manager;


pub struct CyCloudEngine {
    manager: Box<dyn TaskManager>,
}

#[derive(Debug)]
pub enum TaskType {
    Container(ContainerProvisionArgs),
    Native(ActiveUser)
}

#[derive(Debug)]
pub enum TaskStatus {
    Starting,
    Running,
    Stopped,
    Cleaning,
    Cleaned,
    Broken,
}

#[async_trait]
trait TaskManager {
    async fn setup(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    async fn stop(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn cleanup(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn status(&self) -> Result<TaskStatus, Box<dyn std::error::Error>>;
    async fn restart(&self) -> Result<(), Box<dyn std::error::Error>>;

}

#[async_trait]
impl TaskManager for ContainerManager {
    /// Sets up the container with retry logic
    ///
    /// This will:
    /// 1. Check if container exists and is running
    /// 2. Provision if needed using Docker Compose
    /// 3. Start/restart if in wrong state
    /// 4. Retry up to MAX_SETUP_ATTEMPTS times
    ///
    /// # Arguments
    /// * `config` - An enum variant for SetupConfig, containing relevant info for container setup
    ///
    /// # Returns
    /// `Ok(())` if container is running, `Err` on failure
    async fn setup(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.setup_impl().await
    }

    async fn status(&self) -> Result<TaskStatus, Box<dyn std::error::Error>> {
        self.status_impl().await
    }

    async fn restart(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.restart_impl().await
    }

    async fn stop(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.stop_impl().await
    }

    async fn cleanup(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.cleanup_impl().await
    }
}

#[async_trait]
impl TaskManager for NativeManager {
    async fn setup(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.setup_impl() 
    }

    async fn status(&self) -> Result<TaskStatus, Box<dyn std::error::Error>> {
        self.status_impl() 
    }

    async fn restart(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.start_impl()
    }

    async fn stop(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.stop_impl() 
    }
    
    async fn cleanup(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.cleanup_impl() 
    }
}

impl CyCloudEngine {
    /// Creates a new `CyCloudEngine` instance with embedded TaskManager which can represent a
    /// container manager, a native user manager or a VM manager
    ///
    /// # Arguments
    /// * `identifier` - Unique identifier for the container
    ///
    /// # Returns
<<<<<<< HEAD
    /// A new `CyCloudEngine` instance or error
    pub async fn new(task_type: TaskType) -> Result<Self, Box<dyn std::error::Error>> {
    
        let manager: Box<dyn TaskManager> = match task_type {
            TaskType::Native(setup_args) => Box::new(NativeManager::new(setup_args).await?),
            TaskType::Container(setup_args) => Box::new(ContainerManager::new(setup_args).await?),
        };

        Ok( Self { manager } )
=======
    /// A new `CyCloudEngine` instance
    pub fn new(containe_name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            container_name: containe_name.to_string(),
        })
>>>>>>> feature/agent-lib
    }

    /// Get container status, user status or VM status
    pub async fn status(&self) -> Result<TaskStatus, Box<dyn std::error::Error>> {
        println!("Getting CyCloudEngine status...");
        self.manager.status().await
    }

    /// Stop the container, lock the user, shutdown the VM
    pub async fn stop(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Stopping CyCloudEngine...");
        self.manager.stop().await
    }

    /// Restart the container, unlock the user or restart the VM
    pub async fn restart(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Restarting CyCloudEngine...");
        self.manager.restart().await
    }

    /// Setup the engine with a container, dedicated user, or VM
    pub async fn setup(&mut self) -> Result<(), Box<dyn std::error::Error>> {
<<<<<<< HEAD
        println!("Setting up CyCloudEngine...");
        self.manager.setup().await
=======
        let docker = Docker::connect_with_local_defaults()?;

        //This would be for if an image is hosted
        /*
        let mut stream = docker.create_image(
            Some(bollard::query_parameters::CreateImageOptions {
                from_image: Some(image.to_string()),
                ..Default::default()
            }),
            None,
            None,
        );
        while let Some(pull_result) = stream.try_next().await? {
            if let Some(status) = pull_result.status {
                println!("pull: {}", status);
            }
        }
        */

        let mut container_running = false;
        for attempt in 0..MAX_SETUP_ATTEMPTS {
            println!("Attempt {} of {}", attempt, MAX_SETUP_ATTEMPTS);

            let container = docker
                .inspect_container(&self.container_name, None::<InspectContainerOptions>)
                .await
                .ok();

            // Extract ID option without further nesting
            if let Some((container_inspect_response, id)) = container
                .as_ref()
                .and_then(|c| c.id.as_ref().map(|id| (c, id)))
            {
                println!("Found existing container {}", id);

                let container_state = match &container_inspect_response.state {
                    Some(state) => state,
                    None => {
                        println!("Container {} is not running, provisioning...", id);
                        provision_container(PORT, &self.container_name).await?;
                        continue;
                    }
                };

                if let Some(state) = container_state.status {
                    match state {
                        ContainerStateStatusEnum::RUNNING => {
                            println!("Container {} is running", id);
                            container_running = true;
                            break;
                        }
                        ContainerStateStatusEnum::CREATED => {
                            println!("Container {} is already provisioned, starting...", id);
                            if let Err(e) = docker
                                .start_container(
                                    &self.container_name,
                                    None::<StartContainerOptions>,
                                )
                                .await
                            {
                                println!("Failed to start container: {}", e);
                                continue;
                            }
                        }
                        ContainerStateStatusEnum::PAUSED => {
                            println!("Container {} is paused, starting...", id);
                            if let Err(e) = docker.unpause_container(&self.container_name).await {
                                println!("Failed to unpause container: {}", e);
                                continue;
                            }
                        }
                        ContainerStateStatusEnum::RESTARTING => {
                            println!("Container {} is restarting, waiting...", id);
                        }
                        ContainerStateStatusEnum::EXITED => {
                            println!("Container {} was exited, restarting...", id);
                            if let Err(e) = docker
                                .start_container(
                                    &self.container_name,
                                    None::<StartContainerOptions>,
                                )
                                .await
                            {
                                println!("Failed to start container: {}", e);
                                continue;
                            }
                        }
                        ContainerStateStatusEnum::DEAD => {
                            return Err("Unrecoverable error: Miner Container is dead".into());
                        }
                        ContainerStateStatusEnum::EMPTY => {
                            println!("Container {} is empty, reprovisioning...", id);
                            if let Err(e) = provision_container(PORT, &self.container_name).await {
                                println!("Failed to reprovision container: {}", e);
                                continue;
                            }
                        }
                        _ => {
                            println!(
                                "Container {} doesn't have a state assigned, provisioning...",
                                id
                            );
                            if let Err(e) = provision_container(PORT, &self.container_name).await {
                                println!("Failed to reprovision container: {}", e);
                                continue;
                            }
                        }
                    }
                } else {
                    println!("Container {} is not running, provisioning...", id);
                    provision_container(PORT, &self.container_name).await?;
                    continue;
                }
            } else {
                println!(
                    "Could not find container {}, provisioning...",
                    self.container_name
                );
                provision_container(PORT, &self.container_name).await?;
                continue;
            }

            let delay_ms = std::cmp::min(500u64 * attempt as u64, 20_000u64);
            println!("Container does not have the correct state, attempting setup and checking back in {} seconds...", delay_ms/1000);
            tokio::time::sleep(std::time::Duration::from_millis(delay_ms)).await;
        }

        if container_running {
            Ok(())
        } else {
            Err("Unrecoverable error: Could not start inference task container".into())
        }
>>>>>>> feature/agent-lib
    }

    /// Stop the task, erase any containers, users or VMs
    pub async fn kill_engine(&self) -> Result<(), Box<dyn std::error::Error>> {
<<<<<<< HEAD
        println!("Shutting down CyCloudEngine...");
        self.manager.cleanup().await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works_for_container_manager() {
        let container_provision_args = ContainerProvisionArgs {
            container_name: "test123".to_string(),
            ssh_port: 22,
            memory_limit: Some("5000".to_string()),
            cpu_limit: Some(3.5)
        };

        let engine = CyCloudEngine::new(TaskType::Container(container_provision_args)).await;
        assert!(engine.is_ok());
    }

    #[tokio::test]
    async fn it_works_for_native_manager() {
        let active_user = "testuser".to_string();

        let engine = CyCloudEngine::new(TaskType::Native(active_user)).await;
        assert!(engine.is_ok());
    }
}
=======
        println!("Cleaning up inference server");

        let docker = Docker::connect_with_local_defaults()?;

        docker
            .remove_container(
                &self.container_name,
                Some(RemoveContainerOptions {
                    force: true,
                    ..Default::default()
                }),
            )
            .await?;

        println!("Force-removed container {}", &self.container_name);
        Ok(())
    }
}
>>>>>>> feature/agent-lib
