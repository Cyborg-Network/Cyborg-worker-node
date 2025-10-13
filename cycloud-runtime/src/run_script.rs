use std::fs;
use std::process::Command;
use std::os::unix::fs::PermissionsExt;
use rust_embed::EmbeddedFile;

#[derive(rust_embed::RustEmbed)]
#[folder = "scripts/"]
struct Assets;

#[derive(Debug)]
pub struct SetupArgs {
    pub container_name: String,
    pub ssh_port: u16,
    pub ssh_pub_key: String,
    pub cleanup : bool,
}

#[derive(Debug)]
pub enum Script {
    Setup(SetupArgs),
    Reset 
}

impl Script {
    fn file_name(&self) -> &'static str {
        match self {
            Script::Setup(_) => "cycloud_container_setup.sh",
            Script::Reset => "cycloud_container_reset.sh",
        }
    }

    fn args(&self) -> Vec<String> {
        match self {
            Script::Setup(args) => {
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
            Script::Reset => vec![]
        }
    }

    fn get(&self) -> Option<EmbeddedFile> {
        Assets::get(self.file_name())
    }
}

pub fn run_script(script: Script) -> Result<(), Box<dyn std::error::Error>>{
    let embedded_file = script.get().ok_or("Script not found")?;
    
    let temp_path = format!("/tmp/{}", script.file_name());

    if std::path::Path::new(&temp_path).exists() {
        fs::remove_file(&temp_path)?;
    }
    fs::write(&temp_path, embedded_file.data.as_ref())?;
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