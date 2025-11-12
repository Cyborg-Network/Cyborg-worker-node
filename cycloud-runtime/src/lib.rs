use async_trait::async_trait;
use container_manager::{ContainerManager, ContainerProvisionArgs};
use native_manager::{NativeManager, ActiveUser};

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
    /// A new `CyCloudEngine` instance or error
    pub async fn new(task_type: TaskType) -> Result<Self, Box<dyn std::error::Error>> {
    
        let manager: Box<dyn TaskManager> = match task_type {
            TaskType::Native(setup_args) => Box::new(NativeManager::new(setup_args).await?),
            TaskType::Container(setup_args) => Box::new(ContainerManager::new(setup_args).await?),
        };

        Ok( Self { manager } )
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
        println!("Setting up CyCloudEngine...");
        self.manager.setup().await
    }

    /// Stop the task, erase any containers, users or VMs
    pub async fn kill_engine(&self) -> Result<(), Box<dyn std::error::Error>> {
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
