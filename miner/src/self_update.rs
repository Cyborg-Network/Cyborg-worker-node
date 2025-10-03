use std::{
    env,
    fs,
    os::unix::fs::PermissionsExt,
    process::Command,
};

use std::os::unix::process::CommandExt;

use crate::global_config;
use crate::error::{Error, Result};

pub fn try_apply_update_if_available() -> Result<()> {
    let current_exe = env::current_exe()?;
    let staged_update = &global_config::PATHS.update_path;

    if staged_update.exists() {
        println!("New version detected. Attempting self-update...");

        let metadata = fs::metadata(&staged_update)?;
        let permissions = metadata.permissions();
        if permissions.mode() & 0o111 == 0 {
            return Err(Error::Custom("Staged update is not executable".to_string()));
        }

        let tmp_copy = current_exe.with_extension("tmp");

        fs::copy(&staged_update, &tmp_copy)?;

        let mut perms = fs::metadata(&tmp_copy)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&tmp_copy, perms)?;

        let backup = current_exe.with_extension("bak");
        fs::rename(&current_exe, &backup)?;

        fs::rename(&tmp_copy, &current_exe)?;

        let _ = fs::remove_file(&staged_update);

        println!("Update applied. Restarting self with new binary...");

        let e = Command::new(current_exe)
            .args(env::args().skip(1))
            .exec();

        unreachable!("exec failed: {e}");
    } else { 
        println!("No update available.");
    }

    Ok(())
}
