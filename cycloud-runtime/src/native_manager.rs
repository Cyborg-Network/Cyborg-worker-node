use std::path::PathBuf;
use std::fs;
use std::process::Command;
use std::os::unix::fs::PermissionsExt;

#[derive(Debug)]
pub struct ConfigureArgs {
    pub container_name: String,
    pub ssh_port: u16,
}

#[derive(Debug)]
pub struct NativeManager {
    active_user: String,
}

impl NativeManager {
    pub async fn new(username: &str) -> Result<Self, Box<dyn std::error::Error>> {

        Ok(Self {
            active_user: username.to_string(),
        })
    }
    
    pub fn remove_user(self) -> Result<(), Box<dyn std::error::Error>> {
        let user_home = get_user_home(&self.active_user)?;

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
}

pub fn create_user(username: &str) -> Result<(), Box<dyn std::error::Error>> {
    let check_user = Command::new("id")
        .arg(username)
        .output();

    if check_user.is_ok() {
        println!("User {} already exists!", username);
        return Ok(())
    }

    let create_user = Command::new("useradd")
        .args([
            "-m",
            "-s", 
            "/bin/bash",
            username
        ])
        .output()?;

    if !create_user.status.success() {
        let stderr = String::from_utf8_lossy(&create_user.stderr);
        return Err(format!("Failed to create user: {}", stderr).into());
    }

    println!("User {} created successfully", username);

    Ok(())
}

fn get_user_home(username: &str) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("getent")
        .args(["passwd", username])
        .output()?;

    if !output.status.success() {
        return Err(format!("User {} not found", username).into());
    }

    let passwd_entry = String::from_utf8_lossy(&output.stdout);
    // Format: username:x:uid:gid:gecos:home:shell
    let parts: Vec<&str> = passwd_entry.trim().split(':').collect();
    
    if parts.len() >= 6 {
        Ok(parts[5].to_string())
    } else {
        Err(format!("Could not parse home directory for user {}", username).into())
    }
}

pub fn check_sshd(username: &str) -> Result<(), Box<dyn std::error::Error>> {
    let user_home = get_user_home(username)?;

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

fn chown(path: &PathBuf, username: &str) -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("chown")
        .arg(username)
        .arg(path)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Failed to chown {}: {}", path.display(), stderr).into());
    }

    Ok(())
}

fn chown_recursive(path: &PathBuf, username: &str) -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("chown")
        .arg("-R")
        .arg(username)
        .arg(path)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Failed to chown -R {}: {}", path.display(), stderr).into());
    }

    Ok(())
}

#[cfg(test)]
mod tests {}
