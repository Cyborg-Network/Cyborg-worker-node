use subxt::{ClientBuilder, DefaultConfig};

pub struct BlockchainClient {
    client: subxt::Client<DefaultConfig>,
}

impl BlockchainClient {
    pub fn new() -> Self {
        let client = ClientBuilder::<DefaultConfig>::new()
            .set_url("wss://your-chain-url")
            .build()
            .expect("Failed to create client");
        BlockchainClient { client }
    }

    pub async fn call_some_chain_function(&self) {
        // Logic for calling chain functions
    }
}
