use std::{
    env, io::Write, path::PathBuf, process::{Command, Stdio}
};
use std::os::unix::fs::PermissionsExt;

use crate::{builder::validate_miner_type, error::{Error, Result}, global_config};

const INSTALLER: &[u8] = include_bytes!("../../scripts/setup.sh");

pub fn install_self(
    parachain_url: &str,
    miner_type: &str,
    account_seed: &str,
) -> Result<()> {
    validate_miner_type(miner_type)?;

    let mut child = Command::new("bash")
        .arg("-s")
        .arg("install")
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .env("PARACHAIN_URL", parachain_url)
        .env("MINER_TYPE", miner_type)
        .env("ACCOUNT_SEED", account_seed)
        .spawn()?;

    child.stdin.take()
        .ok_or(Error::custom("Failed to get stdin"))?
        .write_all(INSTALLER)?;

    let status = child.wait()
        .map_err(|e| Error::Custom(format!("Failed to wait for installer: {}", e)))?;

    if !status.success() {
        return Err(Error::Custom(format!(
            "Installer failed with exit code: {:?}",
            status.code()
        )));
    }

    Ok(())
}

pub fn try_apply_update_if_available() -> Result<()> {
    let current_version = env!("CARGO_PKG_VERSION");
    println!("Trying to update from version {} to latest.", current_version);
    
    let mut check_cmd = Command::new("bash")
        .arg("-s")
        .arg("check-update")
        .arg(current_version)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    
    check_cmd.stdin.as_mut()
        .ok_or_else(|| Error::from("Failed to open stdin"))?
        .write_all(INSTALLER)?;
    
    let output = check_cmd.wait_with_output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let latest_tag = stdout.trim();
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    if !output.status.success() {
        eprintln!("Update check failed: {stderr}");
        return Ok(());
    }
    
    if latest_tag.is_empty() {
        println!("No update available.");
        return Ok(());
    }
    
    println!("Update available: {}", latest_tag);
    
    let log_file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&global_config::PATHS.updater_log_path)?;
    
    writeln!(&log_file, "\n===== Starting update from {} at {:?} =====", 
             current_version, chrono::Utc::now())?;

    let temp_installer_path = PathBuf::from(&global_config::PATHS.safe_tmp_dir_path)
        .join("updater.sh");

    // To make sure that the updater is always up to date, we re-create it every time
    if temp_installer_path.metadata().is_ok() {
        std::fs::remove_file(&temp_installer_path)?;
    }

    std::fs::write(&temp_installer_path, INSTALLER)?;
    std::fs::set_permissions(&temp_installer_path, std::fs::Permissions::from_mode(0o700))?;

    Command::new("systemd-run")
        .arg("--unit=cyborg-miner-updater")
        .arg("--description=Cyborg Miner Update Process")
        .arg("--collect")
        .arg("bash")
        .arg(&temp_installer_path)
        .arg("update")
        .arg(current_version)
        .arg(latest_tag)
        .stdout(Stdio::from(log_file.try_clone()?))
        .stderr(Stdio::from(log_file))
        .spawn()?
        .wait()?;

    println!("Updater launched, check {} for logs", global_config::PATHS.updater_log_path.display());
    std::process::exit(0);
}
