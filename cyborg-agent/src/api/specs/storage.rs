use std::process::Command;
use std::str;

//disk space needs to be retrieved like this, because the rust crates will return total diskspaces
//and are not filterable by mountpoint like this is, which causes issues on VPS
//this should only be a workaround until we have a better solution
pub fn return_total_storage() -> u64 {
    let output = Command::new("df")
        .arg("--block-size=1")
        .arg("--total")
        .arg("-B1")
        .output()
        .expect("Failed to execute command");

    // Print the raw command output for debugging
    println!("Command output: {}", String::from_utf8_lossy(&output.stdout));

    let stdout = str::from_utf8(&output.stdout).expect("Invalid UTF-8");

    let mut total_space: u64 = 0;

    for line in stdout.lines().skip(1) {  // Skip the header line
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        // Check if the first column (filesystem) starts with "/dev/"
        if let Some(filesystem) = parts.get(0) {
            if filesystem.starts_with("/dev/") {
                if let Some(space) = parts.get(1) {
                    total_space += space.parse::<u64>().unwrap_or(0);
                }
            }
        }
    }

    // Print the total disk space found
    println!("Total disk space from /dev/: {}", total_space);
    total_space
}

