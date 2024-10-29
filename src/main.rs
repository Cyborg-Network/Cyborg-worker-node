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

    match &cli.command {
        Some(Commands::Registration {
            api_url,
            account_seed,
        }) => {
            println!("Registering worker with API URL: {}", api_url);

            let client = CyborgClientBuilder::default()
                .node_uri(api_url.to_string())
                .keypair(account_seed)?
                .build()
                .await?;

            client.register_worker().await?;
        }

        Some(Commands::Startmining {
            api_url,
            account_seed,
            ipfs_url,
        }) => {
            println!(
                "Starting mining session with API URL: {}, IPFS URL: {:?}",
                api_url, ipfs_url
            );

            let client = CyborgClientBuilder::default()
                .node_uri(api_url.to_string())
                .keypair(account_seed)?
                .ipfs_uri(ipfs_url.clone())
                .build()
                .await?;

            client.start_mining_session().await?;
        }

        _ => {
            println!("No command provided. Exiting.");
        }
    }
    Ok(())
}
