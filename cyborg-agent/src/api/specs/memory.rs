use std::process::{Command, Stdio};
use std::str;
use sysinfo::{System, RefreshKind, SystemExt};
use anyhow::{anyhow, Result};

pub async fn get_memory() -> Result<String> {
    let ps_child = Command::new("free") // `ps` command...
        .arg("-h") // with argument `axww`...
        .stdout(Stdio::piped()) // of which we will pipe the output.
        .spawn()?; // Once configured, we actually spawn the command...
                   //
    let grep_child_one = Command::new("grep")
        .arg("-i")
        .arg("Mem")
        .stdin(Stdio::from(ps_child.stdout.ok_or(anyhow!("Failed to get memory from stdout"))?)) // Pipe through.
        .stdout(Stdio::piped())
        .spawn()?;

    let output = grep_child_one.wait_with_output()?;
    let result = str::from_utf8(&output.stdout)?;

    let res = &result.to_string()[14..19];

    Ok(res.to_string())
}

pub fn return_total_memory() -> u64 {
     let system = System::new_with_specifics(
        RefreshKind::new()
            .with_memory()
    );

    system.total_memory() * 1024
}
