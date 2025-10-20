fn main() {
    let required_resources = [
        // Container configuration
        "resources/Dockerfile",
        "resources/docker-compose.yml",
        
        // Shell scripts (these are your renamed original scripts)
        "resources/cycloud_setup_container.sh",        // setup.sh logic
        "resources/cycloud_modify_container_access.sh",    // modify_access.sh logic
        
        // API daemon and service (one-time host setup)
        "resources/cycloud_container_access_control_api.sh",       // container-access-api.sh
        "resources/cycloud-container-access-api.service",  // systemd service
    ];

    for path in &required_resources {
        if !std::fs::metadata(path).is_ok() {
            panic!("Missing required resource: {}", path);
        }
    }

    println!("cargo:rerun-if-changed=resources/");
    
    for path in &required_resources {
        println!("cargo:rerun-if-changed={}", path);
    }
}