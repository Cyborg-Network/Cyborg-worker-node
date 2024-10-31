use crate::worker::{CyborgClient, WorkerData};
use std::error::Error;
use subxt::utils::AccountId32;
use subxt::{OnlineClient, PolkadotConfig};
use subxt_signer::{SecretUri, sr25519::Keypair as SR25519Keypair};
use std::str::FromStr;
use url::Url;
use ipfs_api_backend_hyper::IpfsClient;
use ipfs_api_backend_hyper::TryFromUri;
use std::env;

pub const CONFIG_FILE_NAME: &str = "registered_worker_config.json";

pub const IPFS_DEFAULT_URI: &str =
	"https://8be9886d720942e0be9c10bc4351e9dd:ea84a88bd688458188735bff8c576e90@ipfs.infura.io:5001/api/v0";

pub struct NoKeypair;
pub struct AccountKeypair(SR25519Keypair);

/// A builder pattern for constructing a `CyborgClient` instance.
///
/// This builder allows for flexible configuration of the Cyborg client,
/// including setting the node URI, keypair, and IPFS URI.
pub struct CyborgClientBuilder<Keypair> {
    node_uri: Option<String>,
    keypair: Keypair,
    ipfs_client: Option<IpfsClient>,
    identity: (AccountId32, u64),
    creator: AccountId32,
}

/// Default implementation for the `CyborgClientBuilder` when no keypair is provided.
///
/// This initializes the builder with default values where node URI and IPFS URI are None
/// and the keypair is set to `NoKeypair`.
impl Default for CyborgClientBuilder<NoKeypair> {
    fn default() -> Self {
        CyborgClientBuilder {
            node_uri: None,
            keypair: NoKeypair,
            ipfs_client: None,
            identity: (AccountId32::from([0u8; 32]), 0),
            creator: AccountId32::from([0u8; 32]),
        }
    }
}

impl<Keypair> CyborgClientBuilder<Keypair> {
    /// Sets the node URI for the client to connect to.
    ///
    /// # Arguments
    /// * `url` - A string representing the WebSocket URL of the node.
    pub fn node_uri(mut self, url: String) -> Self {
        self.node_uri = Some(url);
        self
    }

    /// Sets the keypair for the client using a provided seed phrase.
    ///
    /// # Arguments
    /// * `seed` - A string slice representing the seed phrase for generating the keypair.
    ///
    /// # Returns
    /// A `Result` that, if successful, contains a new `CyborgClientBuilder` instance with an `AccountKeypair`.
    pub fn keypair(
        self,
        seed: &str,
    ) -> Result<CyborgClientBuilder<AccountKeypair>, Box<dyn Error>> {
        // TODO add in seed argument instead of hardcoded seed
        let uri = SecretUri::from_str("bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice")?;
        let keypair = SR25519Keypair::from_uri(&uri)?;
        Ok(CyborgClientBuilder {
            node_uri: self.node_uri,
            keypair: AccountKeypair(keypair),
            ipfs_client: self.ipfs_client,
            identity: self.identity,
            creator: self.creator,
        })
    }

    /// Sets the IPFS URI for the client to use.
    ///
    /// # Arguments
    /// * `url` - A string representing the IPFS server URL.
    pub fn ipfs_uri(mut self, url: String) -> Self {
        let ipfs_credentials = env::var("IPFS");

        let ipfs_url = ipfs_credentials
        .ok()
        .and_then(|creds| creds.parse::<Url>().ok())
        .unwrap_or_else(|| {
            IPFS_DEFAULT_URI
                .parse::<Url>()
                .expect("Invalid default IPFS URI")
        });

        let ipfs_client = IpfsClient::build_with_base_uri(ipfs_url.to_string().parse().unwrap())
            .with_credentials(ipfs_url.username(), ipfs_url.password().unwrap());
        
        self.ipfs_client = Some(ipfs_client);
        self
    }

    pub fn config(mut self, config: WorkerData) -> Self {
        self.identity = config.worker;
        self.creator = AccountId32::from_str(&config.creator).unwrap();
        self
    }
}

impl CyborgClientBuilder<AccountKeypair> {
    /// Builds the `CyborgClient` using the provided configurations.
    ///
    /// # Returns
    /// A `Result` that, if successful, contains the constructed `CyborgClient`.
    pub async fn build(self) -> Result<CyborgClient, Box<dyn Error>> {
        match &self.node_uri {
            Some(url) => {
                // Create an online client that connects to the specified Substrate node URL.
                let client = OnlineClient::<PolkadotConfig>::from_url(url).await?;

                Ok(CyborgClient {
                    client,
                    keypair: self.keypair.0,
                    ipfs_client: self.ipfs_client,
                    node_uri: self.node_uri,
                    identity: self.identity,
                    creator: self.creator,
                })
            }
            None => Err("No node URI provided. Please specify a node URI to connect.".into()),
        }
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use clap::builder;
    use sp_core::sr25519;
    use std::error::Error;

    #[tokio::test]
    async fn test_node_uri() {
        // Test setting the node URI in the builder.
        let builder = CyborgClientBuilder::default().node_uri("ws://127.0.0.1:9988".to_string());
        assert_eq!(builder.node_uri, Some("ws://127.0.0.1:9988".to_string()));

        // Test setting both node URI and keypair.
        let builder = CyborgClientBuilder::default()
            .node_uri("ws://127.0.0.1:9988".to_string())
            .keypair("//Alice");

        assert_eq!(
            builder.unwrap().node_uri,
            Some("ws://127.0.0.1:9988".to_string())
        );
    }

    #[tokio::test]
    async fn test_keypair() {
        // Test setting the keypair in the builder.
        let builder = CyborgClientBuilder::default()
            .node_uri("ws://127.0.0.1:9988".to_string())
            .keypair("//Alice");

        let uri_alice = SecretUri::from_str("//Alice").unwrap();
        let expected_public_key = SR25519Keypair::from_uri(&uri_alice)
            .expect("keypair was not set correctly")
            .public_key();
        let uri_bob = SecretUri::from_str("//Bob").unwrap();
        let unexpected_public_key = SR25519Keypair::from_uri(&uri_bob)
            .expect("keypair was not set correctly")
            .public_key();

        if let AccountKeypair(keypair) = builder.unwrap().keypair {
            assert_eq!(keypair.public_key().to_account_id(), expected_public_key.to_account_id());
        } else {
            assert_eq!(unexpected_public_key.to_account_id(), expected_public_key.to_account_id());
        }
    }

    #[tokio::test]
    async fn test_ipfs_uri() -> Result<(), Box<dyn Error>> {
        // Test setting the IPFS URI in the builder.
        let builder = CyborgClientBuilder::default()
            .node_uri("ws://127.0.0.1:9944".to_string())
            .keypair("//Alice")?
            .ipfs_uri("http://127.0.0.1:5001".to_string());

        assert_eq!(builder.ipfs_uri, Some("http://127.0.0.1:5001".to_string()));

        // Test setting the IPFS URI without a keypair.
        let builder = CyborgClientBuilder::default()
            .node_uri("ws://127.0.0.1:9944".to_string())
            .ipfs_uri("http://127.0.0.1:5001".to_string());

        assert_eq!(builder.ipfs_uri, Some("http://127.0.0.1:5001".to_string()));

        Ok(())
    }
}
*/