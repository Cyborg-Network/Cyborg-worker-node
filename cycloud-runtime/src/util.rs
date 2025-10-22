use crate::{
    run_script::{self, Script},
    PORT,
};

pub async fn provision_container(
    port: u16,
    container_name: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    run_script::run_script(Script::Setup)?;

    Ok(())
}

pub async fn remove_container(container_name: &String) -> Result<(), Box<dyn std::error::Error>> {
    run_script::run_script(Script::Reset)?;

    Ok(())
}
