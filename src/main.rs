/// The main function serves as the entry point for the Cyborg Client application.
/// It parses command-line arguments using Clap and executes the corresponding subcommand.
///
/// # Commands:
///
/// - `registration`: Registers a worker with the given blockchain node URL and account seed.
/// - `startmining`: Starts a mining session with the provided blockchain node URL, account seed, and IPFS URL.
///
/// # Errors:
///
/// Returns a `Box<dyn Error>` in case of failure, which could include errors from client building, registration, or mining operations.
///
/// # Usage:
///
/// Run the executable with appropriate subcommands to register or start mining a worker.
mod builder;
mod cli;
mod worker;
mod specs;
mod substrate_interface;
mod zk_helper;

use crate::worker::BlockchainClient;
use builder::CyborgClientBuilder;
use clap::Parser;
use cli::{Cli, Commands};
use std::{env, error::Error, fs};
use zk_helper::{fetch_and_build, generate_trusted_setup};
//use subxt::ext::jsonrpsee::core::client::error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    dotenv::dotenv().ok();

    // Match on the provided subcommand and execute the corresponding action.
    match &cli.command {
        // Handle the "registration" subcommand.
        Some(Commands::Registration {
            parachain_url,
            account_seed,
        }) => {
            println!("Registering worker with API URL: {}", parachain_url);

            // Build the Cyborg client using the provided API URL and account seed.
            let client = CyborgClientBuilder::default()
                .parachain_url(parachain_url.to_string())
                .keypair(account_seed)?
                .build()
                .await?;

            // Register the worker using the built client.
            client.register_worker().await?;
        }

        // Handle the "startmining" subcommand.
        Some(Commands::Startmining {
            parachain_url,
            account_seed,
            //ipfs_url,
        }) => {
            println!("Starting mining session. Parachain URL: {}", parachain_url);

            let current_dir = env::current_dir().expect("Failed to get current directory");
            let config_string = current_dir.join("worker_config.json");
            let slice = config_string.to_str();

            if let Some(slice) = slice {
                
            let config = fs::read_to_string(slice)?;

            let config: worker::WorkerData = serde_json::from_str(&config)?;

            println!("Config: {config:?}");

            // Build the Cyborg client using the provided API URL, account seed, and IPFS URL.
            let client = CyborgClientBuilder::default()
                .parachain_url(parachain_url.to_string())
                .keypair(account_seed)?
                .ipfs_uri().await
                .config(config)
                .build()
                .await?;

            // Start the mining session using the built client.
            client.start_mining_session().await?;
            } else {
                println!("Config file not found. Exiting.");
            }
        }

        _ => {
            println!("No command provided. Exiting.");
        }
    }
    Ok(())
}
