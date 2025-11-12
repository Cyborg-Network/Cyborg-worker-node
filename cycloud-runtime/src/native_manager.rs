use std::path::PathBuf;
use std::fs;
use std::process::Command;
use std::os::unix::fs::PermissionsExt;

use crate::TaskStatus;

pub type ActiveUser = String;

#[derive(Debug)]
pub struct NativeManager {
    active_user: ActiveUser,
}

impl NativeManager {
    pub async fn new(username: String) -> Result<Self, Box<dyn std::error::Error>> {

        Ok(Self {
            active_user: username,
        })
    }

    pub fn setup_impl(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.create_user()?;
        self.check_sshd()?;

        Ok(())
    }

    fn create_user(&self) -> Result<(), Box<dyn std::error::Error>> {
        let check_user = Command::new("id")
            .arg(&self.active_user)
            .output();

        if check_user.is_ok() {
            println!("User {} already exists!", self.active_user);
            return Ok(())
        }

        let create_user = Command::new("useradd")
            .args([
                "-m",
                "-s", 
                "/bin/bash",
                &self.active_user
            ])
            .output()?;

        if !create_user.status.success() {
            let stderr = String::from_utf8_lossy(&create_user.stderr);
            return Err(format!("Failed to create user: {}", stderr).into());
        }

        println!("User {} created successfully", self.active_user);

        Ok(())
    }
    
    pub fn cleanup_impl(&self) -> Result<(), Box<dyn std::error::Error>> {
        let user_home = self.get_user_home()?;

        let remove = Command::new("userdel")
            .arg("-r")
            .arg(&self.active_user)
            .output()?;

        if !remove.status.success() {
            let stderr = String::from_utf8_lossy(&remove.stderr);
            return Err(format!("Failed to remove user {}: {}", &self.active_user, stderr).into());
        }

        //TODO remove potential cronjobs / processes that are not removed by this


        if PathBuf::from(&user_home).exists() {
            return Err("Removing the user from the miner failed: User home directory still exists!".into());
        } else {
            println!("User {} and respective home directory deleted!", self.active_user);
        }

        Ok(())
    }

    fn get_user_home(&self) -> Result<String, Box<dyn std::error::Error>> {
        let output = Command::new("getent")
            .args(["passwd", &self.active_user])
            .output()?;

        if !output.status.success() {
            return Err(format!("User {} not found", &self.active_user).into());
        }

        let passwd_entry = String::from_utf8_lossy(&output.stdout);
        // Format: username:x:uid:gid:gecos:home:shell
        let parts: Vec<&str> = passwd_entry.trim().split(':').collect();
        
        if parts.len() >= 6 {
            Ok(parts[5].to_string())
        } else {
            Err(format!("Could not parse home directory for user {}", &self.active_user).into())
        }
    }

    pub fn check_sshd(&self) -> Result<(), Box<dyn std::error::Error>> {
        let user_home = self.get_user_home()?;

        let sshd_installed = Command::new("sshd")
            .arg("-V")
            .output();

        match sshd_installed {
            Ok(_) => {}
            Err(_) => return Err("sshd is not installed.".into()),
        }

        let status = Command::new("systemctl")
            .args(["is-active", "sshd"])
            .output();

        match status {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                if stdout.trim() != "active" {
                    return Err("sshd is installed but not running.".into())
                }
            }
            Err(_) => return Err("Could not check sshd service status.".into()),
        };

        let ssh_dir = PathBuf::from(user_home).join(".ssh");

        if !ssh_dir.exists() {
            fs::create_dir_all(&ssh_dir)
                .map_err(|e| format!("Failed to create {}: {}", ssh_dir.display(), e))?;
            let mut perms = fs::metadata(&ssh_dir)
                .map_err(|e| format!("Failed to get metadata: {}", e))?
                .permissions();
            perms.set_mode(0o700);
            fs::set_permissions(&ssh_dir, perms)
                .map_err(|e| format!("Failed to set permissions: {}", e))?;
        }

        let auth_keys = ssh_dir.join("authorized_keys");

        if auth_keys.exists() {
            if fs::OpenOptions::new().append(true).open(&auth_keys).is_err() {
                return Err(format!("Cannot write to {}", auth_keys.display()).into());
            }
        } else {
            fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&auth_keys)
                .map_err(|e| format!("Cannot create {}: {}", auth_keys.display(), e))?;
            let mut perms = fs::metadata(&auth_keys)
                .map_err(|e| format!("Failed to get metadata: {}", e))?
                .permissions();
            perms.set_mode(0o600);
            fs::set_permissions(&auth_keys, perms)
                .map_err(|e| format!("Failed to set permissions: {}", e))?;
        }

        Ok(())
    }

    fn sshd_is_active(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let status = Command::new("systemctl")
            .args(["is-active", "sshd"])
            .output();

        match status {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                if stdout.trim() != "active" {
                    return Ok(false)
                }
            }
            Err(_) => return Err("Could not check sshd service status.".into()),
        };

        Ok(true)
    }

    fn user_is_locked(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let output = Command::new("passwd")
            .arg("-S")
            .arg(&self.active_user)
            .output()?;

        let stdout = String::from_utf8_lossy(&output.stdout);


        // TODO this should probably be verified better (eg. does the username itself contain "L"?)
        Ok(stdout.contains(" L "))
    }

    fn sshd_key_exists(&self) -> Result<bool, Box<dyn std::error::Error>> {
        //TODO Check if the user has an ssh key dropped into the vm
        println!("WARNING `sshd_key_exists` is a dummy implementation and will always return true -> TODO!");
        Ok(true)
    }

    fn user_exists(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let status = Command::new("id")
            .arg(&self.active_user)
            .status()?;

        if !status.success() {
            return Ok(false)
        }

        Ok(true)
    }

    pub fn status_impl(&self) -> Result<TaskStatus, Box<dyn std::error::Error>> {
        let sshd_is_active = self.sshd_is_active()?;
        let user_is_locked = self.user_is_locked()?;
        let user_exists = self.user_exists()?;
        let sshd_key_exists = self.sshd_key_exists()?;

        match (sshd_is_active, user_is_locked, user_exists, sshd_key_exists) {
            (false, _, _, _) => Ok(TaskStatus::Broken),
            (_, true, _, _) => Ok(TaskStatus::Stopped),
            (true, false, true, false) => Ok(TaskStatus::Starting),
            (true, false, false, _) => Ok(TaskStatus::Cleaned),
            (true, false, true, true) => Ok(TaskStatus::Running),
        }
    }

    pub fn stop_impl(&self) -> Result<(), Box<dyn std::error::Error>> {
        let output = Command::new("passwd")
            .arg("-l")
            .arg(&self.active_user)
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("WARNING: Failed to lock user, task still running: {}", stderr).into())
        }

        Ok(())
    }

    pub fn start_impl(&self) -> Result<(), Box<dyn std::error::Error>> {
        let output = Command::new("passwd")
            .arg("-u")
            .arg(&self.active_user)
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("CRITICAL: Failed to unlock user, task blocked: {}", stderr).into())
        }

        Ok(())

    }

    #[allow(dead_code)]
    fn chown(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let output = Command::new("chown")
            .arg(&self.active_user)
            .arg(path)
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Failed to chown {}: {}", path.display(), stderr).into());
        }

        Ok(())
    }

    #[allow(dead_code)]
    fn chown_recursive(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let output = Command::new("chown")
            .arg("-R")
            .arg(&self.active_user)
            .arg(path)
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Failed to chown -R {}: {}", path.display(), stderr).into());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {}
