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
use bollard::Docker;
use std::os::unix::fs::PermissionsExt;

pub struct Resource {
    pub content: &'static str,
    pub target: &'static str,
}

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

#[derive(Debug)]
pub struct ProvisionArgs {
    pub container_name: String,
    pub ssh_port: u16,
    pub memory_limit: Option<String>,
    pub cpu_limit: Option<f32>,
}

#[derive(Debug)]
pub struct ConfigureArgs {
    pub container_name: String,
    pub ssh_port: u16,
    pub ssh_pub_key: String,
}

#[derive(Debug)]
pub struct ContainerManager {
    docker: Docker,
}

impl ContainerManager {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            docker: Docker::connect_with_local_defaults()?,
        })
    }

    pub async fn provision_container(
        &self,
        args: ProvisionArgs,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("Provisioning container: {}", args.container_name);

        self.setup_api().await?;
        self.build_image().await?;

        let memory_limit = args.memory_limit.as_deref().unwrap_or("4g");
        let cpu = args.cpu_limit.unwrap_or(2.0);
        let memswap_limit = "4g";
        let pids_limit = 1024;
        let created_date = chrono::Utc::now().to_rfc3339();

        // TODO: replace with something like minijinja for reliable variable substitution
        let compose_content = Resources::DOCKER_COMPOSE.content
            .replace("${SSH_PORT}", &args.ssh_port.to_string())
            .replace("${CONTAINER_NAME}", &args.container_name)
            .replace("${MEMORY_LIMIT}", memory_limit)
            .replace("${CPU_LIMIT}", &cpu.to_string())
            .replace("${MEM_SWAP_LIMIT}", memswap_limit)
            .replace("${PIDS_LIMIT}", &pids_limit.to_string())
            .replace("${CREATED_DATE}", &created_date);

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

                println!("Container {} provisioned successfully", args.container_name);
                Ok(())
            }
        )?;

        Ok(()) 
    }

    async fn build_image(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.docker.inspect_image("cycloud-user-container:latest").await.is_ok() {
            println!("Image cycloud-user-container:latest already exists");
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
                    .arg("cycloud-user-container:latest")
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

    pub async fn configure_ssh(
        &self,
        args: ConfigureArgs,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("Configuring SSH access for: {}", args.container_name);

        use_file(
            &Resources::SETUP_CONTAINER.target.into(), 
            Resources::SETUP_CONTAINER.content, 
            |script_path| {
                let output = Command::new("bash")
                    .arg(&script_path)
                    .arg("--container-name").arg(&args.container_name)
                    .arg("--ssh-public-key").arg(&args.ssh_pub_key)
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

    pub async fn cleanup_container(
        &self,
        container_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("Cleaning up container: {}", container_name);
        
        use_file(
            &Resources::CLEANUP_SCRIPT.target.into(), 
            Resources::CLEANUP_SCRIPT.content, 
            |cleanup_path| {
                let output = Command::new("bash")
                    .arg(cleanup_path)
                    .arg("--container-name").arg(container_name)
                    .arg("--cleanup")
                    .output()?;

                if !output.status.success() {
                    eprintln!("Warning: Cleanup script had errors");
                }

                Ok(())
            }
        )?;

        if let Err(e) = self.stop_container(container_name).await {
            eprintln!("Warning: Failed to stop container: {}", e);
        }

        self.remove_container(container_name).await?;

        let _ = Command::new("docker")
            .arg("volume")
            .arg("rm")
            .arg(format!("{}-home", container_name))
            .arg(format!("{}-workspace", container_name))
            .output();

        println!("Container {} cleaned up successfully", container_name);
        Ok(())
    }

    pub async fn start_container(
        &self,
        container_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("Starting container: {}", container_name);
        self.docker
            .start_container(container_name, None::<StartContainerOptions>)
            .await?;
        Ok(())
    }

    pub async fn stop_container(
        &self,
        container_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("Stopping container: {}", container_name);
        self.docker
            .stop_container(
                container_name,
                Some(StopContainerOptions { t: Some(10), signal: None }),
            )
            .await?;
        Ok(())
    }

    pub async fn remove_container(
        &self,
        container_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("Removing container: {}", container_name);
        self.docker
            .remove_container(
                container_name,
                Some(RemoveContainerOptions {
                    force: true,
                    ..Default::default()
                }),
            )
            .await?;
        Ok(())
    }

    pub async fn restart_container(
        &self,
        container_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("Restarting container: {}", container_name);
        self.stop_container(container_name).await?;
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        self.start_container(container_name).await?;
        Ok(())
    }

    pub async fn get_container_status(
        &self,
        container_name: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let container = self.docker
            .inspect_container(container_name, None::<InspectContainerOptions>)
            .await?;
        
        let status = container.state
            .and_then(|s| s.status)
            .map(|s| format!("{:?}", s))
            .unwrap_or_else(|| "Unknown".to_string());
        
        Ok(status)
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

        let out = Command::new("ls")
            .arg("-l")
            .arg(Resources::CONTAINER_ACCESS_API.target)
            .output()?;

        if !out.status.success() {
            return Err(format!("Failed to check file permissions: {}", String::from_utf8_lossy(&out.stderr)).into());
        } else {
            println!("File permissions checked successfully: {}", String::from_utf8_lossy(&out.stdout)); 
        }



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
        let manager = ContainerManager::new().await;
        assert!(manager.is_ok());
    }

    #[tokio::test]
    async fn test_compose_generation() {
        let manager = ContainerManager::new().await.unwrap();
        let args = ProvisionArgs {
            container_name: "test123".to_string(),
            ssh_port: 2222,
            memory_limit: Some("4g".to_string()),
            cpu_limit: Some(2.0),
        };

        manager.provision_container(args).await.unwrap();

        assert!(Resources::DOCKER_COMPOSE.content.contains("test123"));
        assert!(Resources::DOCKER_COMPOSE.content.contains("mem_limit: 4g"));
        assert!(Resources::DOCKER_COMPOSE.content.contains("cpus: 2"));
    }
}