use async_trait::async_trait;
use futures::StreamExt;
use sp_core::{sr25519::Pair as Sr25519Keypair, Pair};
use std::error::Error;
use subxt::client;
use subxt::events::EventDetails;
use subxt::events::Events;
use subxt::ext::jsonrpsee::async_client::ClientBuilder;
use subxt::{OnlineClient, SubstrateConfig};

#[async_trait]
/// A trait for blockchain client operations, such as registering a worker, starting mining sessions, and processing events.
///
/// Provides an asynchronous API for interacting with a blockchain, which enables clients to register workers,
/// initiate mining sessions, and handle blockchain events with asynchronous operations.
pub trait BlockchainClient {
    /// Registers a worker node on the blockchain.
    ///
    /// # Returns
    /// A `Result` indicating `Ok(())` if successful, or an `Error` if registration fails.
    async fn register_worker(&self) -> Result<(), Box<dyn Error>>;

    /// Starts a mining session on the blockchain by subscribing to events and listening to finalized blocks.
    ///
    /// # Returns
    /// A `Result` indicating `Ok(())` if the session starts successfully, or an `Error` if it fails.
    async fn start_mining_session(&self) -> Result<(), Box<dyn Error>>;

    /// Processes an event received from the blockchain.
    ///
    /// # Arguments
    /// * `event` - A reference to an `EventDetails` object containing details of the blockchain event.
    ///
    /// # Returns
    /// An `Option<String>` containing relevant information derived from the event, or `None` if no information is extracted.
    async fn process_event(&self, event: &EventDetails<SubstrateConfig>) -> Option<String>;
}

/// Represents a client for interacting with the Cyborg blockchain.
///
/// This struct is used to interact with the Cyborg blockchain, manage key pairs,
/// and optionally communicate with IPFS or node URIs.
pub struct CyborgClient {
    pub(crate) client: OnlineClient<SubstrateConfig>,
    pub(crate) keypair: Sr25519Keypair,
    pub ipfs_uri: Option<String>,
    pub node_uri: Option<String>,
}

#[subxt::subxt(runtime_metadata_path = "src/metadata.scale")]
pub mod cyborg_node {} // Contains generated types and APIs for interacting with the Cyborg blockchain runtime.

/// Implementation of the `BlockchainClient` trait for `CyborgClient`.
#[async_trait]
impl BlockchainClient for CyborgClient {
    /// Registers a worker with the Cyborg parachain.
    ///
    /// # Returns
    /// A `Result` indicating success or an error if registration fails.
    async fn register_worker(&self) -> Result<(), Box<dyn Error>> {
        println!("Registering worker with the Cyborg parachain...");
        let call = "";

        //Debug
        println!(
            "parachain url : {:?}",
            self.node_uri.clone().unwrap_or_else(|| "".to_string())
        );
        println!(
            "ipfs url : {:?}",
            self.ipfs_uri.clone().unwrap_or_else(|| "".to_string())
        );

        println!("Worker registration submitted: {:?}", call);
        Ok(())
    }

    /// Starts a mining session by subscribing to finalized blocks and listening for events.
    ///
    /// # Returns
    /// A `Result` indicating success or an error if starting the session fails.
    async fn start_mining_session(&self) -> Result<(), Box<dyn Error>> {
        println!("Starting mining session...");

        //Debug
        println!(
            "parachain url : {:?}",
            self.node_uri.clone().unwrap_or_else(|| "".to_string())
        );
        println!(
            "ipfs url : {:?}",
            self.ipfs_uri.clone().unwrap_or_else(|| "".to_string())
        );

        //let api = OnlineClient::<SubstrateConfig>::from_url("ws://127.0.0.1:9988").await?;
        let api = OnlineClient::<SubstrateConfig>::from_url("ws://127.0.0.1:9988").await?;

        //let mut event_sub = api.events().subscribe_events().await?;
        let all_sub = api.blocks().subscribe_finalized().await?;

        // Subscribe to events from the blockchain
        //let mut event_sub = self.client.events().subscribe().await?;
        println!("Listening for mining session events. Press Ctrl+C to stop.");

        Ok(())
    }

    /// Processes an event from the blockchain.
    ///
    /// # Arguments
    /// * `event` - A reference to an `EventDetails` object containing event information.
    ///
    /// # Returns
    /// An `Option<String>` that may contain information derived from the event.
    async fn process_event(&self, event: &EventDetails<SubstrateConfig>) -> Option<String> {
        //Debug
        println!(
            "parachain url : {:?}",
            self.node_uri.clone().unwrap_or_else(|| "".to_string())
        );
        println!(
            "ipfs url : {:?}",
            self.ipfs_uri.clone().unwrap_or_else(|| "".to_string())
        );

        None
    }
}
