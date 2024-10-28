mod worker;
mod substrate_api;

fn main() {
    let client = Arc::new(substrate_api::substrate_client::BlockchainClient::new());
    worker::worker::start_worker(client);
}
