use crate::worker::CyborgClient;
use sp_core::{sr25519::Pair as Sr25519Keypair, Pair};
use std::error::Error;
use subxt::{storage::DefaultAddress, OnlineClient, SubstrateConfig};

pub struct NoKeypair;
pub struct AccountKeypair(Sr25519Keypair);

pub struct CyborgClientBuilder<Keypair> {
    node_uri: Option<String>,
    keypair: Keypair,
    ipfs_uri: Option<String>,
}

impl Default for CyborgClientBuilder<NoKeypair> {
    fn default() -> Self {
        CyborgClientBuilder {
            node_uri: None,
            keypair: NoKeypair,
            ipfs_uri: None,
        }
    }
}

impl<Keypair> CyborgClientBuilder<Keypair> {
    pub fn node_uri(mut self, url: String) -> Self {
        self.node_uri = Some(url);
        self
    }

    pub fn keypair(
        self,
        seed: &str,
    ) -> Result<CyborgClientBuilder<AccountKeypair>, Box<dyn Error>> {
        let keypair = Sr25519Keypair::from_string(seed, None)?;
        Ok(CyborgClientBuilder {
            node_uri: self.node_uri,
            keypair: AccountKeypair(keypair),
            ipfs_uri: self.ipfs_uri,
        })
    }

    pub fn ipfs_uri(mut self, url: String) -> Self {
        self.ipfs_uri = Some(url);
        self
    }
}

impl CyborgClientBuilder<AccountKeypair> {
    pub async fn build(self) -> Result<CyborgClient, Box<dyn Error>> {
        match &self.node_uri {
            Some(url) => {
                let client = OnlineClient::<SubstrateConfig>::from_url(url).await?;

                Ok(CyborgClient {
                    client,
                    keypair: self.keypair.0,
                    ipfs_uri: self.ipfs_uri,
                    node_uri: self.node_uri,
                })
            }
            None => Err("No node URI provided. Please specify a node URI to connect.".into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::builder;
    use sp_core::sr25519;
    use std::error::Error;

    #[tokio::test]
    async fn test_node_uri() {
        let builder = CyborgClientBuilder::default().node_uri("ws://127.0.0.1:9988".to_string());

        assert_eq!(builder.node_uri, Some("ws://127.0.0.1:9988".to_string()));

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
        let builder = CyborgClientBuilder::default()
            .node_uri("ws://127.0.0.1:9988".to_string())
            .keypair("//Alice");

        let expected_public_key = sr25519::Pair::from_string("//Alice", None)
            .expect("keypair was not set correctly")
            .public();
        let unexpected_public_key = sr25519::Pair::from_string("//Bob", None)
            .expect("keypair was not set correctly")
            .public();

        if let AccountKeypair(keypair) = builder.unwrap().keypair {
            assert_eq!(keypair.public(), expected_public_key);
        } else {
            assert_eq!(unexpected_public_key, expected_public_key);
        }
    }

    #[tokio::test]
    async fn test_ipfs_uri() -> Result<(), Box<dyn Error>> {
        let builder = CyborgClientBuilder::default()
            .node_uri("ws://127.0.0.1:9944".to_string())
            .keypair("//Alice")?
            .ipfs_uri("http://127.0.0.1:5001".to_string());

        assert_eq!(builder.ipfs_uri, Some("http://127.0.0.1:5001".to_string()));

        let builder = CyborgClientBuilder::default()
            .node_uri("ws://127.0.0.1:9944".to_string())
            .ipfs_uri("http://127.0.0.1:5001".to_string());

        assert_eq!(builder.ipfs_uri, Some("http://127.0.0.1:5001".to_string()));

        Ok(())
    }
}
