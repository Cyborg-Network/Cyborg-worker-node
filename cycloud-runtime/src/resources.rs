use std::path::PathBuf;
use std::{fs, vec};
use std::process::Command;
use std::os::unix::fs::PermissionsExt;
use bollard::query_parameters::{RemoveContainerOptions, StartContainerOptions, StopContainerOptions};
use bollard::Docker;
use include_dir::{include_dir, Dir};

static RESOURCES: Dir = include_dir!("$CARGO_MANIFEST_DIR/resources");

#[derive(Debug)]
pub struct ConfigureArgs {
    pub container_name: String,
    pub ssh_port: u16,
    pub ssh_pub_key: String,
    pub cleanup : bool,
}

#[derive(Debug)]
pub struct ProvisionArgs {
    pub container_name: String,
    pub ssh_port: u16,
    pub ssh_pub_key: String,
}

#[derive(Debug)]
pub enum Script {
    Provision(ProvisionArgs),
    Configure(ConfigureArgs),
    Reset 
}

struct ContainerManager {
    docker: Docker,
    container_name: String,
    temp_dir: PathBuf,
}

impl ContainerManager {
    pub async fn new(container_name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let temp_dir: PathBuf = "/tmp".into();

        for file in RESOURCES.files() {
            let temp_path = temp_dir.join(file.path());

            if let Some(parent) = temp_path.parent() {
                fs::create_dir_all(parent)?;
            }

            // If the file already exists, remove it, to safely regenerate it
            if std::path::Path::new(&temp_path).exists() {
                fs::remove_file(&temp_path)?;
            }

            fs::write(&temp_path, file.contents())?;
        }

        Ok(Self {
            docker: Docker::connect_with_local_defaults()?,
            temp_dir,
            container_name: container_name.to_string(),
        })
    }

    pub fn provision(&self) -> Result<(), Box<dyn std::error::Error>> {
        let compose_file = self.temp_dir.join("docker-compose.yml");

        let status = Command::new("docker")
            .arg("compose")
            .arg("-f")
            .arg(compose_file)
            .arg("up")
            .arg("-d")
            .status()?;

        if !status.success() {
            Err("Failed to run docker-compose up".into());
        }

        let script_path = self.temp_dir.join("provision.sh");
        let status = Command::new("bash")
            .arg(script_path)
            .status()?;

        if !status.success() {
            Err("Provision script failed".into());
        }

        Ok(())
    }

    pub async fn start_container(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.docker
            .start_container(&self.container_name, None::<StartContainerOptions>)
            .await?;
        Ok(())
    }

    pub async fn stop_container(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.docker
            .stop_container(
                &self.container_name,
                Some(StopContainerOptions { t: Some(10) }),
            )
            .await?;
        Ok(())
    }

    pub async fn remove_container(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.docker
            .remove_container(
                &self.container_name,
                Some(RemoveContainerOptions {
                    force: true,
                    ..Default::default()
                }),
            )
            .await?;
        Ok(())
    }
}

impl Script {
    fn file_name(&self) -> &'static str {
        match self {
            Script::Configure(_) => "cycloud_configure_container.sh",
            Script::Reset => "cycloud_reset_container.sh",
            Script::Provision(_) => "cycloud_provision_container.sh"
        }
    }

    fn args(&self) -> Vec<String> {
        match self {
            Script::Configure(args) => {
                let mut arg_vec = vec![
                    "--container-name".to_string(),
                    args.container_name.clone(),
                    "--ssh-public-key".to_string(),
                    args.ssh_pub_key.clone(),
                    "--ssh-port".to_string(),
                    args.ssh_port.to_string(),
                ];

                if args.cleanup {
                    arg_vec.push("--cleanup".to_string());
                }

                arg_vec
            }
            Script::Provision(args) => {
                vec![
                    "--container-name".to_string(),
                    args.container_name.clone(),
                    "--ssh-public-key".to_string(),
                    args.ssh_pub_key.clone(),
                    "--ssh-port".to_string(),
                    args.ssh_port.to_string(),
                ]
            },
            Script::Reset => vec![]
        }
    }

    fn get(&self) -> Option<&include_dir::File> {
        RESOURCES.get_file(self.file_name())
    }
}

pub fn run_script(script: Script) -> Result<(), Box<dyn std::error::Error>>{
    let embedded_file = script.get().ok_or("Script not found")?;
    
    let temp_path = format!("/tmp/{}", script.file_name());

    if std::path::Path::new(&temp_path).exists() {
        fs::remove_file(&temp_path)?;
    }
    fs::write(&temp_path, embedded_file.contents())?;
    fs::set_permissions(&temp_path, fs::Permissions::from_mode(0o755))?;

    let status = Command::new(&temp_path)
        .args(script.args())
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err("Failed to run script".into())
    }
}