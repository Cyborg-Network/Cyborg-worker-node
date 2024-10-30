use async_trait::async_trait;
use futures::StreamExt;
use sp_core::{sr25519::Pair as Sr25519Keypair, Pair};
use std::error::Error;
use subxt::book::usage::events;
use subxt::client;
use subxt::events::EventDetails;
use subxt::ext::jsonrpsee::async_client::ClientBuilder;
use subxt::ext::jsonrpsee::core::client::Subscription;
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
    /// A `Result<(), Error>` indicating `Ok(())` if processing is successful, or an `Error` if an issue occurs.
    async fn process_event(
        &self,
        event: &EventDetails<SubstrateConfig>,
    ) -> Result<(), Box<dyn Error>>;
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

        // Subscribe to finalized blocks
        let mut block_subscription = self.client.blocks().subscribe_finalized().await?;
        println!("Listening for mining session events. Press Ctrl+C to stop.");

        // Iterate over each block and process its events
        while let Some(block) = block_subscription.next().await {
            let block = block?;
            let events = block.events().await?;

            // Process each event in the finalized block
            for event in events.iter() {
                if let Ok(event) = event {
                    if let Err(e) = self.process_event(&event).await {
                        println!("Error processing event: {:?}", e)
                    }
                } else {
                    println!("Error decoding event: {:?}", event);
                }
            }
        }

        Ok(())
    }

    /// Processes an event from the blockchain.
    ///
    /// Checks the type of each event and prints relevant details for specific events
    /// such as `WorkerRegistered`, `WorkerRemoved`, and `WorkerStatusUpdated`.
    ///
    /// # Arguments
    /// * `event` - A reference to an `EventDetails` object containing event information.
    ///
    /// # Returns
    /// A `Result<(), Error>` indicating success if the event is processed correctly,
    /// or an error if event decoding fails.
    async fn process_event(
        &self,
        event: &EventDetails<SubstrateConfig>,
    ) -> Result<(), Box<dyn Error>> {
        // subscription_builder.subscribe_to::<cyborg_node::pallet_task_management::events::TaskScheduled>();
        // subscription_builder.subscribe_to::<cyborg_node::pallet_task_management::events::SubmittedCompletedTask>();
        // subscription_builder.subscribe_to::<cyborg_node::pallet_task_management::events::VerifierResolverAssigned>();
        // subscription_builder.subscribe_to::<cyborg_node::pallet_task_management::events::VerifiedCompletedTask>();
        // subscription_builder.subscribe_to::<cyborg_node::pallet_task_management::events::ResolvedCompletedTask>();
        // subscription_builder.subscribe_to::<cyborg_node::pallet_task_management::events::TaskReassigned>();

        // Check for WorkerRegistered event
        match event.as_event::<cyborg_node::edge_connect::events::WorkerRegistered>() {
            Ok(Some(worker_registered)) => {
                let creator = &worker_registered.creator;
                let worker = &worker_registered.worker;
                let domain = &worker_registered.domain;

                // `worker` is a tuple (T::AccountId, WorkerId)
                let (worker_account, worker_id) = &worker_registered.worker;

                println!(
                    "Worker Registered: Creator: {:?}, Worker: {:?}, Domain: {:?}",
                    creator, worker, domain
                );
            }
            Err(e) => {
                println!("Error decoding WorkerRegistered event: {:?}", e);
                return Err(Box::new(e));
            }
            _ => {} // Skip non-matching events
        }

        // Check for WorkerRemoved event
        match event.as_event::<cyborg_node::edge_connect::events::WorkerRemoved>() {
            Ok(Some(worker_removed)) => {
                let creator = &worker_removed.creator;
                let worker_id = &worker_removed.worker_id;

                println!(
                    "Worker Removed: Creator: {:?}, Worker ID: {:?}",
                    creator, worker_id
                );
            }
            Err(e) => {
                println!("Error decoding WorkerRemoved event: {:?}", e);
                return Err(Box::new(e));
            }
            _ => {} // Skip non-matching events
        }

        // Check for WorkerStatusUpdated event
        match event.as_event::<cyborg_node::edge_connect::events::WorkerStatusUpdated>() {
            Ok(Some(status_updated)) => {
                let creator = &status_updated.creator;
                let worker_id = &status_updated.worker_id;
                let worker_status = &status_updated.worker_status;

                println!(
                    "Worker Status Updated: Creator: {:?}, Worker ID: {:?}, Status: {:?}",
                    creator, worker_id, worker_status
                );
            }
            Err(e) => {
                println!("Error decoding WorkerStatusUpdated event: {:?}", e);
                return Err(Box::new(e));
            }
            _ => {} // Skip non-matching events
        }

        Ok(())
    }
}
