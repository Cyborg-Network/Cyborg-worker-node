use bollard::{query_parameters::InspectContainerOptions, secret::ContainerStateStatusEnum};
use bollard::Docker;

mod container_manager;
use container_manager::{ContainerManager, ProvisionArgs, ConfigureArgs};

const MAX_SETUP_ATTEMPTS: u8 = 100;
const SSH_PORT: u16 = 22;

#[derive(Debug)]
pub struct CyCloudEngine {
    container_name: String,
    manager: ContainerManager,
}

impl CyCloudEngine {
    /// Creates a new `CyCloudEngine` instance with embedded ContainerManager
    ///
    /// # Arguments
    /// * `container_name` - Unique identifier for the container
    ///
    /// # Returns
    /// A new `CyCloudEngine` instance or error
    pub async fn new(container_name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let manager = ContainerManager::new().await?;

        Ok(Self {
            container_name: container_name.to_string(),
            manager,
        })
    }

    /// Sets up the container with retry logic
    ///
    /// This will:
    /// 1. Check if container exists and is running
    /// 2. Provision if needed using Docker Compose
    /// 3. Start/restart if in wrong state
    /// 4. Retry up to MAX_SETUP_ATTEMPTS times
    ///
    /// # Arguments
    /// * `ssh_pub_key` - User's SSH public key for access
    /// * `memory_limit` - Optional memory limit (e.g., "4g")
    /// * `cpu_limit` - Optional CPU limit (e.g., 2.0)
    ///
    /// # Returns
    /// `Ok(())` if container is running, `Err` on failure
    pub async fn setup(
        &mut self,
        ssh_pub_key: &str,
        memory_limit: Option<String>,
        cpu_limit: Option<f32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let docker = Docker::connect_with_local_defaults()?;

        let mut container_running = false;

        for attempt in 0..MAX_SETUP_ATTEMPTS {
            println!("Setup attempt {} of {}", attempt + 1, MAX_SETUP_ATTEMPTS);

            let container = docker
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
                        if let Err(e) = self.manager.start_container(&self.container_name).await {
                            println!("Failed to start container: {}", e);
                            continue;
                        }
                    }
                    Some(ContainerStateStatusEnum::PAUSED) => {
                        println!("Container {} is paused, unpausing...", id);
                        if let Err(e) = docker.unpause_container(&self.container_name).await {
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
                        if let Err(e) = self.manager.start_container(&self.container_name).await {
                            println!("Failed to restart container: {}", e);
                            continue;
                        }
                    }
                    Some(ContainerStateStatusEnum::DEAD) => {
                        println!("Container {} is dead, removing and reprovisioning...", id);
                        let _ = self.manager.remove_container(&self.container_name).await;
                        
                        // Provision new container
                        if let Err(e) = self.provision_new_container(
                            ssh_pub_key,
                            memory_limit.clone(),
                            cpu_limit,
                        ).await {
                            println!("Failed to provision container: {}", e);
                            continue;
                        }
                    }
                    Some(ContainerStateStatusEnum::EMPTY) | None | _ => {
                        println!("Container {} has invalid state, reprovisioning...", id);
                        let _ = self.manager.remove_container(&self.container_name).await;
                        
                        if let Err(e) = self.provision_new_container(
                            ssh_pub_key,
                            memory_limit.clone(),
                            cpu_limit,
                        ).await {
                            println!("Failed to provision container: {}", e);
                            continue;
                        }
                    }
                }
            } else {
                println!("Container {} not found, provisioning...", self.container_name);
                if let Err(e) = self.provision_new_container(
                    ssh_pub_key,
                    memory_limit.clone(),
                    cpu_limit,
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

    /// Provision a new container using ContainerManager
    async fn provision_new_container(
        &self,
        ssh_pub_key: &str,
        memory_limit: Option<String>,
        cpu_limit: Option<f32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.manager.ensure_network().await?;

        self.manager.provision_container(ProvisionArgs {
            container_name: self.container_name.clone(),
            ssh_port: SSH_PORT,
            memory_limit,
            cpu_limit,
        }).await?;

        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

        self.manager.configure_ssh(ConfigureArgs {
            container_name: self.container_name.clone(),
            ssh_port: SSH_PORT,
            ssh_pub_key: ssh_pub_key.to_string(),
        }).await?;

        Ok(())
    }

    /// Open a port for the container
    ///
    /// # Arguments
    /// * `protocol` - "tcp", "udp", or "both"
    /// * `port` - Port number or range (e.g., "8080" or "8000-8100")
    /// * `source_ip` - Optional source IP/CIDR restriction
    pub async fn open_port(
        &self,
        protocol: &str,
        port: &str,
        source_ip: Option<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        use container_manager::open_container_port;

        open_container_port(&self.manager, &self.container_name, protocol, port, source_ip).await
    }

    /// Close a port for the container
    ///
    /// # Arguments
    /// * `protocol` - "tcp", "udp", or "both"
    /// * `port` - Port number or range
    pub async fn close_port(
        &self,
        protocol: &str,
        port: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        use container_manager::close_container_port;

        close_container_port(&self.manager, &self.container_name, protocol, port).await
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
        self.manager.cleanup_container(&self.container_name).await?;
        println!("Container {} removed successfully", self.container_name);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_engine_creation() {
        let engine = CyCloudEngine::new("test_container").await;
        assert!(engine.is_ok());
    }
}