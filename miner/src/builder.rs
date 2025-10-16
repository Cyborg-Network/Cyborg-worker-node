use crate::{
    error::{Error, Result}, 
    parachain_interactor::registration::retrieve_identity, 
    substrate_interface::api::runtime_types::cyborg_primitives::miner::MinerType, 
    types::{Miner, ParentRuntime}
};
use std::{str::FromStr, sync::Arc};
use subxt_signer::{sr25519::Keypair as SR25519Keypair, SecretUri};
use tokio::sync::RwLock;

pub struct MinerBuilder {
    miner_type: Option<MinerType>,
    parachain_url: Option<String>,
}

pub struct MinerBuilderStage2 {
    miner_type: MinerType,
    keypair: SR25519Keypair,
    miner_uuid: Vec<u8>,
}

impl MinerBuilder {
    pub fn new() -> Self {
        MinerBuilder {
            miner_type: None,
            parachain_url: None,
        }
    }

    pub fn miner_type(mut self, miner_type: &str) -> Result<Self> {
        self.miner_type = match miner_type {
            "cloud" => Some(MinerType::Cloud),
            "edge" => Some(MinerType::Edge),
            _ => return Err("The supplied miner type is not valid!".into()),
        };
        Ok(self)
    }

    pub fn parachain_url(mut self, url: String) -> Self {
        self.parachain_url = Some(url);
        self
    }

    pub fn keypair(self, seed: &str,miner_uuid: Vec<u8>) -> Result<MinerBuilderStage2> {
        let uri = SecretUri::from_str(seed).map_err(|e| Error::Custom(e.to_string()))?;
        let keypair = SR25519Keypair::from_uri(&uri).map_err(|e| Error::Custom(e.to_string()))?;

        Ok(MinerBuilderStage2 {
            miner_type: self.miner_type.ok_or("Miner type must be set")?,
            keypair,
            miner_uuid,
        })
    }
}

impl MinerBuilderStage2 {
    pub async fn build(self) -> Result<Arc<Miner>> {
        let miner_type = Arc::new(self.miner_type);
        let keypair = Arc::new(self.keypair);

        let miner_identity = retrieve_identity(
            Arc::clone(&keypair), 
            Arc::clone(&miner_type),
            self.miner_uuid.clone(),
        ).await?;

        Ok(Arc::new(Miner {
            miner_type,
            parent_runtime: Arc::new(RwLock::new(ParentRuntime { port: None })),
            keypair,
            identity: Arc::new(miner_identity),
            current_task: Arc::new(RwLock::new(None))
        }))
    }
}

#[cfg(test)]
mod tests { }
