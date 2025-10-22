use std::{
    env,
    process::{Command, Stdio},
    io::Write,
};

use crate::error::{Error, Result};

const INSTALLER: &[u8] = include_bytes!("../../scripts/setup.sh");

pub fn install_self() -> Result<()> {
    let mut child = Command::new("sh")
        .arg("-s") // read commands from stdin
        .arg("install")
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    {
        let stdin = child.stdin.as_mut().ok_or("Failed to get stdin")?;
        stdin.write_all(INSTALLER)?;
    }

    let output = child.wait_with_output()?;

    if output.status.success() {
        Ok(())
    } else {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(Error::Custom(format!("Installer failed (exit: {:?}). stdout: {}\nstderr: {}", output.status.code(), stdout, stderr)))
    }
}

pub fn try_apply_update_if_available() -> Result<()> {
    // TODO: validate binary before updating with checksum or version
    // Depending on time taken to update, we might need to add a call to the parachain to set miner to inactive before update and to active after

    // We get the current version of the binary:
    let current_version = env!("CARGO_PKG_VERSION");

    println!("Trying to update from version {} to latest.", current_version);

    let mut child = Command::new("sh")
        .arg("-s") // read commands from stdin
        .arg("update")
        .arg(current_version)
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    {
        let stdin = child.stdin.as_mut().ok_or("Failed to get stdin")?;
        stdin.write_all(INSTALLER)?;
    }

    let output = child.wait_with_output()?;

    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Error::Custom(format!("Update failed (exit: {:?}). stdout: {}\nstderr: {}", output.status.code(), stdout, stderr)))
    }

    Ok(())
}
