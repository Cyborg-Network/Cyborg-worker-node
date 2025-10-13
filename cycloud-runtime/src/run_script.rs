use std::fs;
use std::process::Command;
use std::os::unix::fs::PermissionsExt;
use rust_embed::EmbeddedFile;

#[derive(rust_embed::RustEmbed)]
#[folder = "scripts/"]
struct Assets;

#[derive(Debug)]
pub enum Script {
    Setup,
    Reset 
}

impl Script {
    fn file_name(&self) -> &'static str {
        match self {
            Script::Setup => "cycloud_container_setup.sh",
            Script::Reset => "cycloud_container_reset.sh",
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
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err("Failed to run script".into())
    }
}