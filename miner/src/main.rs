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
mod error;
mod global_config;
mod log;
mod miner_types;
mod parachain_interactor;
mod parent_runtime;
mod self_management;
mod specs;
mod tests;
mod traits;
mod utils;

use builder::MinerBuilder;
use clap::Parser;
use cli::{Cli, Commands};
use cyborg_agent::{run_agent, AgentConfig};
use error::Result;
use global_config::run_global_config;
use std::{process::Command, sync::Arc};
use tokio::time::{sleep, Duration};
use traits::ParachainInteractor;
use types::substrate_interface::api::edge_connect::calls::types::remove_miner::MinerId;
use types::substrate_interface::api::runtime_types::bounded_collections::bounded_vec::BoundedVec;

#[derive(serde::Deserialize)]
pub struct IpResponse {
    pub ip: String,
}

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

            // Run the agent with config
            let agent_config = Arc::new(AgentConfig {
                current_task: Arc::clone(&miner.current_task),
                log_file_path: &global_config::PATHS.log_path,
                container_prefix: &global_config::CONTAINER_PREFIX,
            });
            tokio::spawn(supervise_agent(agent_config));

            // Start the mining session using the built miner.
            miner.start_miner().await?;
        }

        Some(Commands::Install {
            parachain_url,
            account_seed,
            miner_type,
            domain_name,
            tailscale_network,
            miner_uuid,
        }) => {
            if let Some(domain_name) = domain_name {
                self_management::install_self(
                    parachain_url,
                    miner_type,
                    account_seed,
                    domain_name,
                    miner_uuid,
                )
                .expect("Failed to install");

                return Ok(());
            }

            if let Some(tailscale_network) = tailscale_network {
                let hostname_output = Command::new("hostname")
                    .output()
                    .expect("Failed to get hostname")
                    .stdout;

                let hostname = String::from_utf8(hostname_output)
                    .expect("Failed to get hostname")
                    .trim()
                    .to_string();

                let domain_name = format!("https://{hostname}.{}", tailscale_network);

                self_management::install_self(
                    parachain_url,
                    miner_type,
                    account_seed,
                    &domain_name,
                    miner_uuid,
                )
                .expect("Failed to install");

                return Ok(());
            }

            let domain_name = reqwest::get("https://api.ipify.org?format=json")
                .await
                .expect("Failed to get IP address")
                .json::<IpResponse>()
                .await
                .expect("Failed to get IP adress")
                .ip;

            self_management::install_self(
                parachain_url,
                miner_type,
                account_seed,
                &domain_name,
                miner_uuid,
            )
            .expect("Failed to install");

            return Ok(());
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

async fn supervise_agent(agent_config: Arc<AgentConfig>) {
    loop {
        let handle = tokio::spawn(run_agent(Arc::clone(&agent_config)));

        match handle.await {
            Ok(Ok(())) => {
                println!("Agent exited cleanly.");
                break;
            }
            Ok(Err(e)) => {
                eprintln!("Agent failed: {e:?}");
            }
            Err(join_err) => {
                eprintln!("Agent task panicked: {join_err}");
            }
        }

        sleep(Duration::from_millis(500)).await;

        println!("Restarting agent...");
    }
}
