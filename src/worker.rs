use async_trait::async_trait;
use futures::StreamExt;
use sp_core::{sr25519::Pair as Sr25519Keypair, Pair};
use std::error::Error;
use subxt::client;
use subxt::events::EventDetails;
use subxt::events::Events;
use subxt::ext::jsonrpsee::async_client::ClientBuilder;
use subxt::{OnlineClient, SubstrateConfig};
//use subxt::config::substrate::SubstrateConfig;

#[async_trait]
pub trait BlockchainClient {
    async fn register_worker(&self) -> Result<(), Box<dyn Error>>;
    async fn start_mining_session(&self) -> Result<(), Box<dyn Error>>;
    async fn process_event(&self, event: &EventDetails<SubstrateConfig>) -> Option<String>;
}

pub struct CyborgClient {
    pub(crate) client: OnlineClient<SubstrateConfig>,
    pub(crate) keypair: Sr25519Keypair,
    pub ipfs_uri: Option<String>,
    pub node_uri: Option<String>,
}

#[subxt::subxt(runtime_metadata_path = "src/metadata.scale")]
pub mod cyborg_node {}

#[async_trait]
impl BlockchainClient for CyborgClient {
    async fn register_worker(&self) -> Result<(), Box<dyn Error>> {
        println!("Registering worker with the Cyborg parachain...");
        let call = "";

        println!("Worker registration submitted: {:?}", call);
        Ok(())
    }

    async fn start_mining_session(&self) -> Result<(), Box<dyn Error>> {
        println!("Starting mining session...");
        //let api = OnlineClient::<SubstrateConfig>::from_url("ws://127.0.0.1:9988").await?;
        let api = OnlineClient::<SubstrateConfig>::from_url("ws://127.0.0.1:9988").await?;

        //let mut event_sub = api.events().subscribe_events().await?;
        let all_sub = api.blocks().subscribe_finalized().await?;

        // Subscribe to events from the blockchain
        //let mut event_sub = self.client.events().subscribe().await?;
        println!("Listening for mining session events. Press Ctrl+C to stop.");

        Ok(())
    }

    async fn process_event(&self, event: &EventDetails<SubstrateConfig>) -> Option<String> {
        None
    }
}
