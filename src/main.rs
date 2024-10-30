mod worker;
mod substrate_api;

#[subxt::subxt(runtime_metadata_path = "substrate_interface.scale")]
pub mod cyborg_metadata {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    worker::start_worker().await?;

    Ok(())
}
