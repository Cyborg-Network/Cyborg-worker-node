use crate::run_script::{self, Script, SetupArgs};

pub async fn provision_container(port: u16, container_name: &String) -> Result<(), Box<dyn std::error::Error>> {
    run_script::run_script(Script::Setup(SetupArgs {
        container_name: container_name.clone(),
        ssh_port: port,
        ssh_pub_key: "key".to_string(),
        cleanup: false
    }))?;

    Ok(())
}

pub async fn remove_container(container_name: &String) -> Result<(), Box<dyn std::error::Error>> {
    run_script::run_script(Script::Reset)?;

    Ok(())
}