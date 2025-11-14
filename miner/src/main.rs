/// The main function serves as the entry point for the Cyborg Miner application.
/// It parses command-line arguments using Clap and executes the corresponding subcommand.
///
/// # Commands:
///
/// - `startminer`: Starts a mining session with the provided parachain URL URL, and account seed
///
/// # Errors:
///
/// Returns a `Box<dyn Error>` in case of failure, which could include errors from client building, registration, or mining operations.
///
/// # Usage:
///
/// Run the executable with appropriate arguments to start mining.
mod builder;
mod cli;
mod global_config;
mod error;
mod log;
mod parachain_interactor;
mod parent_runtime;
mod specs;
mod substrate_interface;
mod traits;
mod types;
mod self_management;
mod utils;

use builder::MinerBuilder;
use clap::Parser;
use cli::{Cli, Commands};
use error::Result;
use global_config::run_global_config;
use traits::ParachainInteractor;
use crate::substrate_interface::api::edge_connect::calls::types::remove_miner::MinerId;
use crate::substrate_interface::api::runtime_types::bounded_collections::bounded_vec::BoundedVec;


#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Match on the provided subcommand and execute the corresponding action.
    match &cli.command {
        // Handle the "start_miner" subcommand.
        Some(Commands::StartMiner {
            parachain_url,
            account_seed,
            miner_type,
            miner_uuid,
        }) => {
            // This is done separately from the miner, as these likely will remain constant, even when running multiple miners
            // Fails fast, an error here is unrecoverable
            run_global_config(parachain_url)
                .await
                .expect("Error running the global config!");

            // Initialize logger
            log::init_logger().expect("Could not initialize logger!");
            
        

            let miner_id_bytes = miner_uuid.as_bytes().to_vec();
            let miner_uuid_bounded: MinerId = BoundedVec(miner_id_bytes);
                

            // Fails fast, an error here is unrecoverable
            let miner = MinerBuilder::new()
                .miner_type(miner_type)
                .expect("Failed to set miner type")
                .parachain_url(parachain_url.to_string())
                .keypair(account_seed, miner_uuid_bounded)
                .expect("Failed to set keypair")
                .build()
                .await
                .expect("Failed to build miner");

            // Start the mining session using the built miner.
            miner.start_miner().await?;
        }

        Some(Commands::Install {
            parachain_url,
            account_seed,
            miner_type,
        }) => {
            self_management::install_self(parachain_url, miner_type, account_seed)
                .expect("Failed to install");
        }

        Some(Commands::Version) => {
            println!("Version: {}", env!("CARGO_PKG_VERSION"));
        }

        _ => {
            println!("No command provided. Exiting.");
        }
    }

    Ok(())
}
