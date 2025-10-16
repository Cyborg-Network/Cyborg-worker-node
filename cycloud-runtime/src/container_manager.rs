use std::path::PathBuf;
use std::fs;
use std::process::Command;
use std::os::unix::fs::PermissionsExt;
use bollard::query_parameters::{InspectContainerOptions, InspectNetworkOptions, RemoveContainerOptions, StartContainerOptions, StopContainerOptions};
use bollard::Docker;
use include_dir::{include_dir, Dir};

static RESOURCES: Dir = include_dir!("$CARGO_MANIFEST_DIR/resources");

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
pub struct ModifyAccessArgs {
    pub container_name: String,
    pub action: String,
    pub protocol: Option<String>,
    pub port: Option<String>,
    pub source_ip: Option<String>,
}

#[derive(Debug)]
pub struct ContainerManager {
    docker: Docker,
    resources_dir: PathBuf,
}

impl ContainerManager {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let resources_dir = PathBuf::from("/tmp/cycloud-resources");
        
        fs::create_dir_all(&resources_dir)?;

        for file in RESOURCES.files() {
            let dest_path = resources_dir.join(file.path());

            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent)?;
            }

            fs::write(&dest_path, file.contents())?;
            
            if dest_path.extension().and_then(|s| s.to_str()) == Some("sh") {
                fs::set_permissions(&dest_path, fs::Permissions::from_mode(0o755))?;
            }
        }

        Ok(Self {
            docker: Docker::connect_with_local_defaults()?,
            resources_dir,
        })
    }

    pub async fn provision_container(
        &self,
        args: ProvisionArgs,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("Provisioning container: {}", args.container_name);

        self.build_image().await?;

        let compose_content = self.generate_compose_file(&args)?;
        let compose_path = self.resources_dir.join(format!("docker-compose-{}.yml", args.container_name));
        fs::write(&compose_path, compose_content)?;

        let output = Command::new("docker")
            .arg("compose")
            .arg("-f")
            .arg(&compose_path)
            .arg("up")
            .arg("-d")
            .current_dir(&self.resources_dir)
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Failed to run docker-compose up: {}", stderr).into());
        }

        println!("Container {} provisioned successfully", args.container_name);
        Ok(())
    }

    async fn build_image(&self) -> Result<(), Box<dyn std::error::Error>> {
        let dockerfile_path = self.resources_dir.join("Dockerfile.container");
        
        if !dockerfile_path.exists() {
            return Err("Dockerfile.container not found in resources".into());
        }

        if self.docker.inspect_image("cycloud-user-container:latest").await.is_ok() {
            println!("Image cycloud-user-container:latest already exists");
            return Ok(());
        }

        println!("Building Docker image...");

        let output = Command::new("docker")
            .arg("build")
            .arg("-t")
            .arg("cycloud-user-container:latest")
            .arg("-f")
            .arg(&dockerfile_path)
            .arg(&self.resources_dir)
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Failed to build Docker image: {}", stderr).into());
        }

        println!("Docker image built successfully");
        Ok(())
    }

    fn generate_compose_file(&self, args: &ProvisionArgs) -> Result<String, Box<dyn std::error::Error>> {
        let template_path = self.resources_dir.join("docker-compose.template.yml");
        
        if !template_path.exists() {
            return Err("docker-compose.template.yml not found in resources".into());
        }

        let template = fs::read_to_string(template_path)?;
        
        let memory = args.memory_limit.as_deref().unwrap_or("4g");
        let cpu = args.cpu_limit.unwrap_or(2.0);

        // Perform variable substitution
        let compose_content = template
            .replace("${CONTAINER_NAME}", &args.container_name)
            .replace("${MEMORY_LIMIT}", memory)
            .replace("${CPU_LIMIT}", &cpu.to_string());

        Ok(compose_content)
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

        let script_path = self.resources_dir.join("cycloud_configure_container.sh");
        
        if !script_path.exists() {
            return Err("Configure script not found".into());
        }

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

    pub async fn modify_access(
        &self,
        args: ModifyAccessArgs,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let script_path = self.resources_dir.join("cycloud_modify_container_access.sh");
        
        if !script_path.exists() {
            return Err("Modify access script not found".into());
        }

        let mut cmd = Command::new("bash");
        cmd.arg(&script_path)
           .arg("--container-name").arg(&args.container_name)
           .arg("--action").arg(&args.action)
           .arg("--force"); // Skip confirmation prompts

        if let Some(protocol) = &args.protocol {
            cmd.arg("--protocol").arg(protocol);
        }

        if let Some(port) = &args.port {
            cmd.arg("--port").arg(port);
        }

        if let Some(source) = &args.source_ip {
            cmd.arg("--source-ip").arg(source);
        }

        let output = cmd.output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Failed to modify access: {}", stderr).into());
        }

        Ok(())
    }

    pub async fn cleanup_container(
        &self,
        container_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("Cleaning up container: {}", container_name);

        let script_path = self.resources_dir.join("cycloud_configure_container.sh");
        
        if script_path.exists() {
            let output = Command::new("bash")
                .arg(&script_path)
                .arg("--container-name").arg(container_name)
                .arg("--cleanup")
                .output()?;

            if !output.status.success() {
                eprintln!("Warning: Cleanup script had errors");
            }
        }

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

        // Remove compose file
        let compose_path = self.resources_dir.join(format!("docker-compose-{}.yml", container_name));
        let _ = fs::remove_file(compose_path);

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
}

pub async fn open_container_port(
    manager: &ContainerManager,
    container_name: &str,
    protocol: &str,
    port: &str,
    source_ip: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    
    manager.modify_access(ModifyAccessArgs {
        container_name: container_name.to_string(),
        action: "open".to_string(),
        protocol: Some(protocol.to_string()),
        port: Some(port.to_string()),
        source_ip,
    }).await?;

    Ok(())
}

pub async fn close_container_port(
    manager: &ContainerManager,
    container_name: &str,
    protocol: &str,
    port: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    
    manager.modify_access(ModifyAccessArgs {
        container_name: container_name.to_string(),
        action: "close".to_string(),
        protocol: Some(protocol.to_string()),
        port: Some(port.to_string()),
        source_ip: None,
    }).await?;

    Ok(())
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

        let compose = manager.generate_compose_file(&args).unwrap();
        assert!(compose.contains("test123"));
        assert!(compose.contains("mem_limit: 4g"));
        assert!(compose.contains("cpus: 2"));
    }
}