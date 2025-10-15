use crate::resources::{self, ConfigureArgs, ProvisionArgs, Script};

pub async fn provision_container(port: u16, container_name: &String) -> Result<(), Box<dyn std::error::Error>> {
    resources::run_script(Script::Provision(ProvisionArgs {
        container_name: container_name.clone(), 
        ssh_port: port,
        ssh_pub_key: "key".to_string()
    }))?;

    resources::run_script(Script::Configure(ConfigureArgs {
        container_name: container_name.clone(),
        ssh_port: port,
        ssh_pub_key: "key".to_string(),
        cleanup: false
    }))?;

    Ok(())
}

pub async fn remove_container(container_name: &String) -> Result<(), Box<dyn std::error::Error>> {
    resources::run_script(Script::Reset)?;

    Ok(())
}