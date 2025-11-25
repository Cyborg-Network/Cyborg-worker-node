use async_trait::async_trait;
use container_manager::ContainerManager;
use native_manager::NativeManager;
use types::{substrate_interface::api::runtime_types::cyborg_primitives::task::CyCloudTask, TaskType};

mod container_manager;
mod native_manager;

pub struct CyCloudEngine {
    manager: Box<dyn TaskManager>,
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
trait TaskManager: Send + Sync {
    async fn setup(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    async fn stop(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn cleanup(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn status(&self) -> Result<TaskStatus, Box<dyn std::error::Error>>;
    async fn restart(&self) -> Result<(), Box<dyn std::error::Error>>;

}

#[async_trait]
impl TaskManager for ContainerManager<'_> {
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
    pub async fn new(task_type: &TaskType) -> Result<Self, Box<dyn std::error::Error>> {

        let cycloud_task = match task_type {
            TaskType::CyCloud(cycloud_task) => cycloud_task,
            _ => return Err("Error creating cycloud runtime: Wrong task type!".into())
        };
    
        let manager: Box<dyn TaskManager> = match cycloud_task {
            CyCloudTask::Native(native_task) => Box::new(
                NativeManager::new(
                    String::from_utf8_lossy(&native_task.user_name.0).to_string()
                ).await?
            ),
            CyCloudTask::Container(_) => Box::new(
                ContainerManager::new(
                    container_manager::ContainerProvisionArgs { 
                        ssh_port: None, 
                        memory_limit: None, 
                        cpu_limit: None, 
                        container_name: None,
                    }
                ).await?
            ),
            CyCloudTask::Vm(_vm_task) => return Err("Error setting up cycloud engine: Vm deployment is not available yet!".into())
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
    use types::substrate_interface::api::runtime_types::{bounded_collections::bounded_vec::BoundedVec, cyborg_primitives::task::{CyCloudContainerTask, CyCloudNativeTask}};

    use super::*;

    #[tokio::test]
    async fn it_works_for_container_manager() {
        let engine = CyCloudEngine::new(
            &TaskType::CyCloud(CyCloudTask::Container(CyCloudContainerTask { _marker: () }))
        ).await;
        assert!(engine.is_ok());
    }

    #[tokio::test]
    async fn it_works_for_native_manager() {
        let active_user = BoundedVec(b"user".to_vec());

        let engine = CyCloudEngine::new(
            &TaskType::CyCloud(CyCloudTask::Native(CyCloudNativeTask { user_name: active_user }))
        ).await;
        assert!(engine.is_ok());
    }
}
