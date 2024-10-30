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

use crate::worker::BlockchainClient;
use builder::CyborgClientBuilder;
use clap::Parser;
use cli::{Cli, Commands};
use std::error::Error;
use subxt::ext::jsonrpsee::core::client::error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    // Match on the provided subcommand and execute the corresponding action.
    match &cli.command {
        // Handle the "registration" subcommand.
        Some(Commands::Registration {
            api_url,
            account_seed,
        }) => {
            println!("Registering worker with API URL: {}", api_url);

            // Build the Cyborg client using the provided API URL and account seed.
            let client = CyborgClientBuilder::default()
                .node_uri(api_url.to_string())
                .keypair(account_seed)?
                .build()
                .await?;

            // Register the worker using the built client.
            client.register_worker().await?;
        }

        // Handle the "startmining" subcommand.
        Some(Commands::Startmining {
            api_url,
            account_seed,
            ipfs_url,
        }) => {
            println!(
                "Starting mining session with API URL: {}, IPFS URL: {:?}",
                api_url, ipfs_url
            );

            // Build the Cyborg client using the provided API URL, account seed, and IPFS URL.
            let client = CyborgClientBuilder::default()
                .node_uri(api_url.to_string())
                .keypair(account_seed)?
                .ipfs_uri(ipfs_url.clone())
                .build()
                .await?;

            // Start the mining session using the built client.
            client.start_mining_session().await?;
        }

        _ => {
            println!("No command provided. Exiting.");
        }
    }
    Ok(())
}
