use std::{
    env,
    process::{Command, Stdio},
    io::Write,
};
use std::os::unix::process::CommandExt;

use crate::{builder::validate_miner_type, error::{Error, Result}, global_config};

const INSTALLER: &[u8] = include_bytes!("../../scripts/setup.sh");

pub fn install_self(
    parachain_url: &str,
    miner_type: &str,
    account_seed: &str,
) -> Result<()> {
    validate_miner_type(miner_type)?;

    let mut child = Command::new("bash")
        .arg("-s") // read commands from stdin
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

    let output = child.wait_with_output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        return Err(Error::Custom(format!(
            "Installer failed (exit: {:?}).\nstdout:\n{}\nstderr:\n{}",
            output.status.code(),
            stdout,
            stderr
        )));
    } else {
        println!("{}", stdout);
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
    
    if stdout.is_empty() {
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
    
    unsafe {
        let mut update_cmd = Command::new("bash")
            .arg("-s")
            .arg("update")
            .arg(current_version)
            .arg(latest_tag)
            .stdin(Stdio::piped())
            .stdout(Stdio::from(log_file.try_clone()?))
            .stderr(Stdio::from(log_file))
            .pre_exec(|| {
                libc::setsid();
                Ok(())
            })
            .spawn()?;

        update_cmd.stdin.take()
            .ok_or_else(|| Error::from("Failed to open stdin"))?
            .write_all(INSTALLER)?;
    }
    
    println!("Updater launched, check {} for logs", global_config::PATHS.updater_log_path.display());
    std::process::exit(0);
}
