use std::sync::Arc;

use crate::{
    error::Result,
    global_config,
    parachain_interactor::{behavior_control, event_processor, registration, task_management},
    parent_runtime::{inference, proof, setup},
    types::{CurrentTask, Miner, ParentRuntime},
};
use async_trait::async_trait;
use subxt::events::EventDetails;
use subxt::PolkadotConfig;
use tokio::sync::RwLock;

#[async_trait]
pub trait InferenceServer {
    /// Downloads a model archive (containing the model and potential additional data eg. proving key) from CESS
    ///
    /// # Arguments
    /// * `task` - A `TaskKind` which the function processes in the appropriate manner
    ///
    /// # Returns
    /// A `Result` containing `Ok(())` if the task is successfully processed, or an `Error` if it fails.
    async fn process_task(&self, task: Arc<RwLock<CurrentTask>>) -> Result<()>;

    /// Starts performing inference, selecting the correct inference engine based on the task type
    ///
    /// # Arguments
    /// * `input` - An `impl Stream<Item = Result<Message, tungstenite::Error>> + Unpin` representing the input stream of messages.
    ///
    /// # Returns
    /// An `impl Stream<Item = Result<Message, tungstenite::Error>>` representing the output stream of messages.
    async fn spawn_inference_server(
        &self,
        current_task: Arc<RwLock<CurrentTask>>,
    ) -> Result</*JoinHandle<()>*/ ()>;

    /// Generates a zkml proof for the model currently in execution.
    ///
    /// # Returns
    /// A `Result` containing a vector of bytes representing the proof.
    async fn generate_proof(&self) -> Result<Vec<u8>>;
}

#[async_trait]
impl InferenceServer for ParentRuntime {
    async fn process_task(&self, task: Arc<RwLock<CurrentTask>>) -> Result<()> {
        setup::process_task(task).await
    }

    async fn spawn_inference_server(
        &self,
        current_task: Arc<RwLock<CurrentTask>>,
    ) -> Result</*JoinHandle<()>*/ ()> {
        inference::spawn_inference_server(current_task, self.port).await
    }

    async fn generate_proof(&self) -> Result<Vec<u8>> {
        proof::generate_proof().await
    }
}

#[async_trait]
/// A trait for blockchain client operations, such as registering a worker, starting mining sessions, and processing events.
///
/// Provides an asynchronous API for interacting with a blockchain, which enables clients to register workers,
/// initiate mining sessions, and handle blockchain events with asynchronous operations.
pub trait ParachainInteractor {
    /// Starts a miner by subscribing to events and listening to finalized blocks.
    ///
    /// # Returns
    /// A `Result` indicating `Ok(())` if the session starts successfully, or an `Error` if it fails.
    async fn start_miner(&self) -> Result<()>;

    /// Processes an event received from the blockchain.
    ///
    /// # Arguments
    /// * `event` - A reference to an `EventDetails` object containing details of the blockchain event.
    ///
    /// # Returns
    /// An `Option<String>` containing relevant information derived from the event, or `None` if no information is extracted.
    async fn process_event(&self, event: &EventDetails<PolkadotConfig>) -> Result<()>;

    /// Submits a zkml (Zero Knowledge Machine Learning) proof to the blockchain.
    ///
    /// # Arguments
    /// * `proof` - A `Vec<u8>` containing the zkml proof.
    ///
    /// # Returns
    /// A `Result` indicating `Ok(())` if the result is successfully submitted, or an `Error` if it fails.
    async fn submit_zkml_proof(&self, proof: Vec<u8>) -> Result<()>;

    /// Vacates a miner erasing current user data and resetting the miner state.
    ///
    /// # Returns
    /// A `Result` indicating `Ok(())` if the session vacates successfully, or an `Error` if it fails.
    async fn stop_task_and_vacate_miner(&self) -> Result<()>;

    /// Attempts to update the miner identity file.
    ///
    /// # Arguments
    /// * `file_path` - A `&str` representing the path to the config file.
    /// * `content` - A `&str` representing the content to be written to the config file.
    fn update_identity_file(&self, path: &str, content: &str) -> Result<()>;

    //TODO this might also notify the user that the miner has been corrupted and that the current task should be pulled
    /// Suspends the miner by sending a transaction to the parachain that deactivates the miner for further tasks..
    ///
    /// # Returns
    /// A `Result` indicating `Ok(())` if the miner is successfully suspended, or an `Error` if it fails.
    async fn suspend_miner(&self) -> Result<()>;

    /// Updates the operational status on the parachain (non-blocking)
    ///
    /// # Arguments
    /// * `status` - The new operational status to set
    ///
    /// # Returns
    /// A `Result` indicating `Ok(())` if successful, or an `Error` if it fails.
    async fn update_operational_status(
        &self,
        status: crate::substrate_interface::api::runtime_types::cyborg_primitives::miner::OperationalStatus,
    ) -> Result<()>;
}

/// Implementation of `ParachainInteractor` trait for `Miner`.
#[async_trait]
impl ParachainInteractor for Arc<Miner> {
    async fn start_miner(&self) -> Result<()> {
        registration::start_miner(Arc::clone(self)).await
    }

    async fn process_event(&self, event: &EventDetails<PolkadotConfig>) -> Result<()> {
        event_processor::process_event(Arc::clone(self), event).await
    }

    async fn stop_task_and_vacate_miner(&self) -> Result<()> {
        task_management::stop_task_and_vacate_miner().await
    }

    async fn submit_zkml_proof(&self, proof: Vec<u8>) -> Result<()> {
        task_management::submit_zkml_proof(Arc::clone(self), proof).await
    }

    fn update_identity_file(&self, path: &str, content: &str) -> Result<()> {
        global_config::update_config_file(path, content)
    }

    async fn suspend_miner(&self) -> Result<()> {
        behavior_control::_miner_self_suspend(self).await
    }

    async fn update_operational_status(
        &self,
        status: crate::substrate_interface::api::runtime_types::cyborg_primitives::miner::OperationalStatus,
    ) -> Result<()> {
        behavior_control::update_operational_status(Arc::clone(self), status).await
    }
}
