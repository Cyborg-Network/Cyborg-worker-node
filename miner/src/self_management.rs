use std::{
    env,
    process::{Command, Stdio},
    io::Write,
};

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

    {
        let stdin = child.stdin.as_mut().ok_or(Error::custom("Failed to get stdin"))?;
        stdin.write_all(INSTALLER)?;
    }

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
    // TODO: validate binary before updating with checksum or version
    // Depending on time taken to update, we might need to add a call to the parachain to set miner to inactive before update and to active after

    let current_version = env!("CARGO_PKG_VERSION");
    println!("Trying to update from version {} to latest.", current_version);

    let mut installer_path = std::env::temp_dir();
    installer_path.push("update.sh");
    std::fs::write(&installer_path, INSTALLER)?;

    let output = Command::new("bash")
        .arg(&installer_path)
        .arg("check-update")
        .arg(current_version)
        .output()?;

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

    writeln!(&log_file, "\n===== Starting update from {} at {:?} =====", current_version, chrono::Utc::now())?;

    Command::new("bash")
        .arg(&installer_path) // read commands from stdin
        .arg("update")
        .arg(current_version)
        .arg(latest_tag)
        .stdin(Stdio::null())
        .stdout(Stdio::from(log_file.try_clone()?))
        .stderr(Stdio::from(log_file))
        .spawn()?;

    println!("Updater launched, check {} for logs", global_config::PATHS.updater_log_path.display());

    std::process::exit(0);
}
