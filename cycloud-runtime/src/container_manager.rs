use std::path::PathBuf;
use std::fs;
use std::process::Command;
use bollard::query_parameters::{
    InspectContainerOptions, 
    InspectNetworkOptions, 
    RemoveContainerOptions, 
    StartContainerOptions, 
    StopContainerOptions
};
use bollard::secret::ContainerStateStatusEnum;
use bollard::Docker;
use std::os::unix::fs::PermissionsExt;

use crate::TaskStatus;

type MemoryLimit = Option<String>;
type CpuLimit = Option<f32>;
type ContainerName<'a> = &'a str;
type SshPort = u16;

pub struct Resource {
    pub content: &'static str,
    pub target: &'static str,
}

#[derive(Debug)]
pub struct ContainerProvisionArgs<'a>{
    pub ssh_port: Option<SshPort>,
    pub memory_limit: MemoryLimit,
    pub cpu_limit: CpuLimit,
    pub container_name: Option<ContainerName<'a>>,
}

#[derive(Debug)]
pub struct ConfigureSshArgs<'a>{
    pub ssh_port: SshPort,
    pub container_name: ContainerName<'a>,
}

#[derive(Debug)]
pub struct ContainerManager<'a> {
    pub docker: Docker,
    pub container_name: ContainerName<'a>,
    pub memory_limit: MemoryLimit,
    pub cpu_limit: CpuLimit,
    ssh_port: SshPort,
}

const SSH_PORT: u16 = 2222;
const DOCKER_IMAGE_NAME: &str = "cycloud-user-container:local";
const MAX_SETUP_ATTEMPTS: u8 = 100;
const DEFAULT_CONTAINER_NAME: &str = "cycloud-user-container";

// We have this function with a closure to make sure that the file is fresh each time (to avoid eg. stale files after updates or removed temp files)
fn use_file<F>(
    file_path: &PathBuf, 
    file_content: &str, 
    action: F
) -> Result<(), Box<dyn std::error::Error>> 
where
    F: Fn(&PathBuf) -> Result<(), Box<dyn std::error::Error>>
{
    if file_path.metadata().is_ok() {
        fs::remove_file(&file_path)?;
    }

    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(file_path, file_content)?;

    action(file_path)?;

    Ok(())
}

macro_rules! define_resources {
    ($($ident:ident => { filename: $filename:expr, target: $target:expr }),* $(,)?) => {
        pub struct Resources;

        impl Resources {
            $(
                pub const $ident: Resource = Resource {
                    content: include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/docker/", $filename)),
                    target: concat!($target, "/", $filename),
                };
            )*
        }
    };
}

//TODO Replace tmp with controlled path or tempfile
define_resources!(
    DOCKERFILE => { filename: "Dockerfile", target: "/tmp/cycloud-resources" },
    DOCKER_COMPOSE => {filename: "docker-compose.yml", target: "/tmp/cycloud-resources"},
    CONTAINER_ACCESS_API_SERVICE => {filename: "cycloud-container-access-api.service", target: "/etc/systemd/system"},
    CLEANUP_SCRIPT => {filename: "cycloud_cleanup_container.sh", target: "/tmp/cycloud-resources"},
    CONTAINER_ACCESS_API => {filename: "cycloud_container_access_control_api.sh", target: "/opt/container-management"},
    MODIFY_CONTAINER_ACCESS => {filename: "cycloud_modify_container_access.sh", target: "/opt/container-management"},
    SETUP_CONTAINER => {filename: "cycloud_setup_container.sh", target: "/tmp/cycloud-resources"},
);

impl<'a> ContainerManager<'a> {
    pub async fn new(setup_args: ContainerProvisionArgs<'a>) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            docker: Docker::connect_with_local_defaults()?,
            container_name: setup_args.container_name.unwrap_or(DEFAULT_CONTAINER_NAME),
            ssh_port: setup_args.ssh_port.unwrap_or(SSH_PORT),
            memory_limit: setup_args.memory_limit,
            cpu_limit: setup_args.cpu_limit,
        })
    }

    pub async fn setup_impl(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut container_running = false;

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
                        if let Err(e) = self.start_container().await {
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
                        if let Err(e) = self.start_container().await {
                            println!("Failed to restart container: {}", e);
                            continue;
                        }
                    }
                    Some(ContainerStateStatusEnum::DEAD) => {
                        println!("Container {} is dead, removing and reprovisioning...", id);
                        let _ = self.remove_container().await;
                        
                        // Provision new container
                        if let Err(e) = self.provision_new_container().await {
                            println!("Failed to provision container: {}", e);
                            continue;
                        }
                    }
                    Some(ContainerStateStatusEnum::EMPTY) | None | _ => {
                        println!("Container {} has invalid state, reprovisioning...", id);
                        let _ = self.remove_container().await;
                        
                        if let Err(e) = self.provision_new_container().await {
                            println!("Failed to provision container: {}", e);
                            continue;
                        }
                    }
                }
            } else {
                println!("Container {} not found, provisioning...", self.container_name);
                if let Err(e) = self.provision_new_container().await {
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
    pub async fn provision_new_container(
        &self,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.ensure_network().await?;

        self.provision_container().await?;

        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

        self.configure_ssh(ConfigureSshArgs {
            container_name: &self.container_name,
            ssh_port: self.ssh_port,
        }).await?;

        Ok(())
    }

    pub async fn provision_container(
        &self,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("Provisioning container: {}", self.container_name);
        let network_name = "container-network";

        self.setup_api().await?;
        self.build_image().await?;

        if let Ok(_) = self.docker.inspect_network(network_name, None::<InspectNetworkOptions>).await {
            self.docker.remove_network(network_name).await?;
        }

        let memory_limit = self.memory_limit.as_deref().unwrap_or("4g");
        let cpu = self.cpu_limit.unwrap_or(2.0);
        let memswap_limit = "4g";
        let pids_limit = 1024;
        let created_date = chrono::Utc::now().to_rfc3339();
        let user_id = uuid::Uuid::new_v4().to_string();

        // TODO: replace with something like minijinja for reliable variable substitution
        let compose_content = Resources::DOCKER_COMPOSE.content
            .replace("${SSH_PORT}", &self.ssh_port.to_string())
            .replace("${CONTAINER_NAME}", &self.container_name)
            .replace("${MEM_LIMIT}", memory_limit)
            .replace("${CPU_LIMIT}", &cpu.to_string())
            .replace("${MEM_SWAP_LIMIT}", memswap_limit)
            .replace("${PIDS_LIMIT}", &pids_limit.to_string())
            .replace("${CREATED_DATE}", &created_date)
            .replace("${USER_ID}", &user_id)
            .replace("${NETWORK}", network_name)
            .replace("${DOCKER_IMAGE_NAME}", DOCKER_IMAGE_NAME);

        use_file(
            &Resources::DOCKER_COMPOSE.target.into(), 
            &compose_content,
            |compose_path| {
                let output = Command::new("docker")
                    .arg("compose")
                    .arg("-f")
                    .arg(&compose_path)
                    .arg("up")
                    .arg("-d")
                    .output()?;

                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    return Err(format!("Failed to run docker-compose up: {}", stderr).into());
                }

                println!("Container {} provisioned successfully", self.container_name);
                Ok(())
            }
        )?;

        Ok(()) 
    }

    async fn build_image(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.docker.inspect_image(DOCKER_IMAGE_NAME).await.is_ok() {
            println!("Image {} already exists", DOCKER_IMAGE_NAME);
            return Ok(());
        }

        println!("Building Docker image...");

        use_file(
            &Resources::DOCKERFILE.target.into(), 
            Resources::DOCKERFILE.content,
            |dockerfile_path| {
                let output = Command::new("docker")
                    .arg("build")
                    .arg("-t")
                    .arg(DOCKER_IMAGE_NAME)
                    .arg("-f")
                    .arg(dockerfile_path)
                    .arg(dockerfile_path.parent().ok_or("Failed to get parent directory")?)
                    .output()?;

                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    return Err(format!("Failed to build Docker image: {}", stderr).into());
                }

                println!("Docker image built successfully");
                Ok(())
            }
        )?;

        Ok(())
    }

    pub async fn ensure_network(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Check if network exists
        if self.docker.inspect_network("container-network", None::<InspectNetworkOptions>).await.is_ok() {
            return Ok(());
        }

        println!("Creating container-network...");

        let output = Command::new("docker")
            .arg("network")
            .arg("create")
            .arg("--driver")
            .arg("bridge")
            .arg("--subnet")
            .arg("172.20.0.0/16")
            .arg("container-network")
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Failed to create network: {}", stderr).into());
        }

        Ok(())
    }

    pub async fn configure_ssh<'b>(
        &self,
        args: ConfigureSshArgs<'a>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("Configuring SSH access for: {}", args.container_name);

        use_file(
            &Resources::SETUP_CONTAINER.target.into(), 
            Resources::SETUP_CONTAINER.content, 
            |script_path| {
                let output = Command::new("bash")
                    .arg(&script_path)
                    .arg("--container-name").arg(&args.container_name)
                    .arg("--ssh-port").arg(args.ssh_port.to_string())
                    .output()?;

                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    return Err(format!("Failed to configure SSH: {}", stderr).into());
                }

                println!("SSH configured successfully for {}", args.container_name);
                Ok(())
            }
        )?; 

        Ok(())
    }

    pub async fn cleanup_impl(
        &self,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("Cleaning up container: {}", &self.container_name);
        
        use_file(
            &Resources::CLEANUP_SCRIPT.target.into(), 
            Resources::CLEANUP_SCRIPT.content, 
            |cleanup_path| {
                let output = Command::new("bash")
                    .arg(cleanup_path)
                    .arg("--container-name").arg(&self.container_name)
                    .arg("--cleanup")
                    .output()?;

                if !output.status.success() {
                    eprintln!("Warning: Cleanup script had errors");
                }

                Ok(())
            }
        )?;

        if let Err(e) = self.stop_impl().await {
            eprintln!("Warning: Failed to stop container: {}", e);
        }

        self.remove_container().await?;

        let _ = Command::new("docker")
            .arg("volume")
            .arg("rm")
            .arg(format!("{}-home", &self.container_name))
            .arg(format!("{}-workspace", &self.container_name))
            .output();

        println!("Container {} cleaned up successfully", &self.container_name);
        Ok(())
    }

    /// Starts the CyCloud container
    pub async fn start_container(
        &self,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("Starting container: {}", &self.container_name);
        self.docker
            .start_container(&self.container_name, None::<StartContainerOptions>)
            .await?;
        Ok(())
    }

    /// Stops the CyCloud container
    pub async fn stop_impl(
        &self,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("Stopping container: {}", self.container_name);
        self.docker
            .stop_container(
                &self.container_name,
                Some(StopContainerOptions { t: Some(10), signal: None }),
            )
            .await?;
        Ok(())
    }

    pub async fn remove_container(
        &self,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("Cleaning up container: {}", self.container_name);

        self.docker
            .remove_container(
                &self.container_name,
                Some(RemoveContainerOptions {
                    force: true,
                    ..Default::default()
                }),
            )
            .await?;

        println!("Container {} removed successfully", self.container_name);

        Ok(())
    }

    pub async fn restart_impl(
        &self,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("Restarting container: {}", &self.container_name);
        self.stop_impl().await?;
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        self.start_container().await?;
        Ok(())
    }

    pub async fn status_impl(
        &self,
    ) -> Result<TaskStatus, Box<dyn std::error::Error>> {
        let container = self.docker
            .inspect_container(&self.container_name, None::<InspectContainerOptions>)
            .await?;
        
        let status = match container.state.and_then(|s| s.status) {
            Some(s) => s,
            None => {
                println!("Unable to retrieve status for container {}, assuming task is broken!", &self.container_name);
                return Ok(TaskStatus::Cleaned)
            }
        };

        println!("The container status check returned the following: {}", status);

        match status {
            ContainerStateStatusEnum::EXITED | ContainerStateStatusEnum::PAUSED | ContainerStateStatusEnum::CREATED => {
                Ok(TaskStatus::Stopped)
            },
            ContainerStateStatusEnum::RUNNING => {
                Ok(TaskStatus::Running)
            },
            ContainerStateStatusEnum::REMOVING => {
                Ok(TaskStatus::Cleaning)
            }
            ContainerStateStatusEnum::DEAD | ContainerStateStatusEnum::EMPTY => {
                Ok(TaskStatus::Broken)
            },
            ContainerStateStatusEnum::RESTARTING => {
                Ok(TaskStatus::Starting)
            }
        }
    }
    
    pub async fn setup_api(
        &self,
    ) -> Result<(), Box<dyn std::error::Error>> {
        use_file(
            &Resources::CONTAINER_ACCESS_API.target.into(), 
            Resources::CONTAINER_ACCESS_API.content, 
            |api_path| {
                let mut permissions = fs::metadata(api_path)?
                    .permissions();
                permissions.set_mode(0o755);
                fs::set_permissions(api_path, permissions)?;

                Ok(())
            }
        )?;

        use_file(
            &Resources::MODIFY_CONTAINER_ACCESS.target.into(), 
            Resources::MODIFY_CONTAINER_ACCESS.content, 
            |modify_path| {
                let mut permissions = fs::metadata(modify_path)?
                    .permissions();
                permissions.set_mode(0o755);
                fs::set_permissions(modify_path, permissions)?;

                Ok(())
            }
        )?;

        use_file(
            &Resources::CONTAINER_ACCESS_API_SERVICE.target.into(), 
            Resources::CONTAINER_ACCESS_API_SERVICE.content, 
            |service_path| {
                let unit_name = service_path
                    .file_name()
                    .ok_or("Invalid unit path")?
                    .to_str()
                    .ok_or("Invalid unit name")?;

                Command::new("systemctl")
                    .arg("daemon-reload")
                    .output()?;

                Command::new("systemctl")
                    .arg("enable")
                    .arg(unit_name)
                    .output()?;

                let restart_output = Command::new("systemctl")
                    .arg("restart")
                    .arg(unit_name)
                    .output()?;

                if !restart_output.status.success() {
                    let start_output = Command::new("systemctl")
                        .arg("start")
                        .arg(unit_name)
                        .output()?;

                    if !start_output.status.success() {
                        return Err(format!("Failed to start container access API service: {}", String::from_utf8_lossy(&start_output.stderr)).into());
                    }
                }

                Ok(())
            }
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_container_manager_init() {
        let args = ContainerProvisionArgs {
            container_name: Some(&"test123".to_string()),
            ssh_port: Some(2222),
            memory_limit: Some("4g".to_string()),
            cpu_limit: Some(2.0),
        };

        let manager = ContainerManager::new(args).await;
        assert!(manager.is_ok());
    }

    #[tokio::test]
    async fn test_compose_generation() {
        let args = ContainerProvisionArgs {
            container_name: None,
            ssh_port: Some(2222),
            memory_limit: Some("4g".to_string()),
            cpu_limit: Some(2.0),
        };

        let manager = ContainerManager::new(args).await.unwrap();

        manager.provision_container().await.unwrap();

        assert!(Resources::DOCKER_COMPOSE.content.contains("test123"));
        assert!(Resources::DOCKER_COMPOSE.content.contains("mem_limit: 4g"));
        assert!(Resources::DOCKER_COMPOSE.content.contains("cpus: 2"));
    }
}
