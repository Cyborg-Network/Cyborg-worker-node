use std::sync::{Arc, RwLock};

use serde::Serialize;
use async_recursion::async_recursion;

use crate::{
    error_handling::ClientError, 
    crypto::encrypt_message
};

use super::{health_status, location, specs};

#[derive(Serialize)]
pub struct Init{
    location: location::Location,
    status: bool,
    specs: specs::Specs,
}

impl Init {
    #[async_recursion]
    async fn get_init() -> Result<Init, anyhow::Error> {
        let location = location::Location::get_location().await?;
        let status = health_status::HealthStatus::get_health_status().await?;
        let specs = specs::Specs::get_specs().await?;

        Ok(
            Init{
                location,
                status,
                specs,
            }
        )
    }

    pub async fn return_init_message(
        diffie_hellman_key: &Arc<RwLock<Option<[u8; 32]>>>
    ) -> Result<String, ClientError> {
        let diffie_hellman_key_copy = {
            let diffie_hellman_key_guard = diffie_hellman_key.read()
                .map_err(|e| ClientError::InitError(e.to_string()))?;
     
            if let Some(key) = *diffie_hellman_key_guard {
                key
            } else {
                return Err(ClientError::AuthError("No diffie hellman key found".to_string()));
            }
        };
     
        let init_item = Init::get_init().await
            .map_err(|e| ClientError::InitError(e.to_string()))?;
     
        let init_item = serde_json::to_string(&init_item)
            .map_err(|e| ClientError::InitError(e.to_string()))?;
     
        let encrypted_message = encrypt_message("Init", &diffie_hellman_key_copy, init_item);
     
        let encrypted_message = serde_json::to_string(&encrypted_message)
            .map_err(|e| ClientError::InitError(e.to_string()))?;
     
        println!("Sending init message: {:?}", encrypted_message);
     
        Ok(encrypted_message)
    }
}
