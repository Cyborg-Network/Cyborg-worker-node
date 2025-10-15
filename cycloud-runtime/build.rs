fn main() {
    for path in [
        "resources/docker-compose.yml",
        "resources/Dockerfile",
        "resources/cycloud_setup_container.sh",
        "resources/cycloud_cleanup_container.sh",
        "resources/cycloud_modify_container_access.sh",
        "resources/cycloud_container_access_control_api.sh",
        "resources/cycloud-container-access-api.service",
    ] {
        if !std::fs::metadata(path).is_ok() {
            panic!("Missing required resource: {}", path);
        }
    }

    println!("cargo:rerun-if-changed=resources/");
}
