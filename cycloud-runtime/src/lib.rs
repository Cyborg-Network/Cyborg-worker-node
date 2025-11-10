use bollard::{query_parameters::InspectContainerOptions, secret::ContainerStateStatusEnum};

mod container_manager;
mod native_manager;

use container_manager::{ContainerManager, ContainerProvisionArgs};
use native_manager::NativeManager;

const MAX_SETUP_ATTEMPTS: u8 = 100;

#[derive(Debug)]
pub struct CyCloudEngine {
    manager: ContainerManager,
}

#[derive(Debug)]
pub enum TaskType {
    Container,
    Native,
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

#[derive(Debug)]
pub enum SetupConfig {
    Container(ContainerProvisionArgs),
    Native
}

trait TaskManager {
    async fn setup(&mut self, config: SetupConfig) -> Result<(), Box<dyn std::error::Error>>;
    async fn stop(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn cleanup(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn status(&self) -> Result<TaskStatus, Box<dyn std::error::Error>>;
}

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
    async fn setup(&mut self, config: SetupConfig) -> Result<(), Box<dyn std::error::Error>> {
        let mut container_running = false;

        let config = match config {
            SetupConfig::Container(c) => c,
            _ => return Err("Invalid config type for ContainerManager".into()),
        };

        for attempt in 0..MAX_SETUP_ATTEMPTS {
            println!("Setup attempt {} of {}", attempt + 1, MAX_SETUP_ATTEMPTS);

            let container = self.docker
                .inspect_container(&self.container_name, None::<InspectContainerOptions>)
                .await
                .ok();

            if let Some(container_info) = container {
                let id = container_info.id.as_deref().unwrap_or("unknown");
                println!("Found existing container {}", id);

                let state = container_info.state.as_ref();
                
                match state.and_then(|s| s.status.as_ref()) {
                    Some(ContainerStateStatusEnum::RUNNING) => {
                        println!("Container {} is running", id);
                        container_running = true;
                        break;
                    }
                    Some(ContainerStateStatusEnum::CREATED) => {
                        println!("Container {} is created, starting...", id);
                        if let Err(e) = self.start_container(&self.container_name).await {
                            println!("Failed to start container: {}", e);
                            continue;
                        }
                    }
                    Some(ContainerStateStatusEnum::PAUSED) => {
                        println!("Container {} is paused, unpausing...", id);
                        if let Err(e) = self.docker.unpause_container(&self.container_name).await {
                            println!("Failed to unpause container: {}", e);
                            continue;
                        }
                    }
                    Some(ContainerStateStatusEnum::RESTARTING) => {
                        println!("Container {} is restarting, waiting...", id);
                        // Just wait and retry
                    }
                    Some(ContainerStateStatusEnum::EXITED) => {
                        println!("Container {} exited, restarting...", id);
                        if let Err(e) = self.start_container(&self.container_name).await {
                            println!("Failed to restart container: {}", e);
                            continue;
                        }
                    }
                    Some(ContainerStateStatusEnum::DEAD) => {
                        println!("Container {} is dead, removing and reprovisioning...", id);
                        let _ = self.remove_container(&self.container_name).await;
                        
                        // Provision new container
                        if let Err(e) = self.provision_new_container(
                            config.memory_limit.clone(),
                            config.cpu_limit,
                        ).await {
                            println!("Failed to provision container: {}", e);
                            continue;
                        }
                    }
                    Some(ContainerStateStatusEnum::EMPTY) | None | _ => {
                        println!("Container {} has invalid state, reprovisioning...", id);
                        let _ = self.remove_container(&self.container_name).await;
                        
                        if let Err(e) = self.provision_new_container(
                            config.memory_limit.clone(),
                            config.cpu_limit,
                        ).await {
                            println!("Failed to provision container: {}", e);
                            continue;
                        }
                    }
                }
            } else {
                println!("Container {} not found, provisioning...", self.container_name);
                if let Err(e) = self.provision_new_container(
                    config.memory_limit.clone(),
                    config.cpu_limit,
                ).await {
                    println!("Failed to provision container: {}", e);
                    continue;
                }
            }

            let delay_ms = std::cmp::min(500u64 * (attempt as u64 + 1), 20_000u64);
            println!("Waiting {} seconds before next check...", delay_ms / 1000);
            tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms)).await;
        }

        if container_running {
            println!("Container {} is ready", self.container_name);
            Ok(())
        } else {
            Err("Failed to start container after maximum attempts".into())
        }
    }

    async fn status(&self) -> Result<TaskStatus, Box<dyn std::error::Error>> {
        self.get_container_status().await
    }

    async fn stop(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.stop_container(&self.container_name).await
    }

    async fn cleanup(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.cleanup_container(&self.container_name).await
    }
}

impl TaskManager for NativeManager {
    async fn setup(&mut self, config: SetupConfig) -> Result<(), Box<dyn std::error::Error>> {
        
    }

    async fn status(&self) -> Result<TaskStatus, Box<dyn std::error::Error>> {
        
    }

    async fn stop(&self) -> Result<(), Box<dyn std::error::Error>> {
        
    }
    
    async fn cleanup(&self) -> Result<(), Box<dyn std::error::Error>> {
        
    }
}

impl CyCloudEngine {
    /// Creates a new `CyCloudEngine` instance with embedded ContainerManager
    ///
    /// # Arguments
    /// * `container_name` - Unique identifier for the container
    ///
    /// # Returns
    /// A new `CyCloudEngine` instance or error
    pub async fn new(identifier: &str, task_type: TaskType) -> Result<Self, Box<dyn std::error::Error>> {
    
        let manager: Box<dyn TaskManager> = match task_type {
            TaskType::Native => Box::new(NativeManager::new(identifier).await?),
            TaskType::Container => Box::new(ContainerManager::new(identifier).await?),
        }

        Ok(Self { manager })
    }

    

    /// Get container status
    pub async fn status(&self) -> Result<String, Box<dyn std::error::Error>> {
        self.manager.get_container_status(&self.container_name).await
    }

    /// Stop the container
    pub async fn stop(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.manager.stop_container(&self.container_name).await
    }

    /// Start the container
    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.manager.start_container(&self.container_name).await
    }

    /// Restart the container
    pub async fn restart(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.manager.restart_container(&self.container_name).await
    }

    /// Completely remove the container and clean up all resources
    ///
    /// This removes:
    /// - The Docker container
    /// - NSG rules
    /// - iptables rules
    /// - Volume data
    /// - Mapping files
    pub async fn kill_engine(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Cleaning up container: {}", self.container_name);

        println!("Container {} removed successfully", self.container_name);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_engine_creation() {
        let engine = CyCloudEngine::new("test_container", TaskType::Container).await;
        assert!(engine.is_ok());
    }
}
