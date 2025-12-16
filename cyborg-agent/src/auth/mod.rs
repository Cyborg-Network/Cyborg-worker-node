use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use x25519_dalek::PublicKey;

use crate::api::logs;
use crate::crypto::{
    compute_diffie_hellman_secret, generate_server_ephemeral_keypair, verify_signature,
};
use crate::error_handling::ClientError;

#[derive(Deserialize, Serialize, Debug)]
pub struct WsAuthRequest {
    pub target_ip: String,
    pub task_id: u64,
    pub signed_timestamp: String,
    pub signed_timestamp_signature: String,
    pub ephemeral_public_key: String,
}

pub struct ProcessedAuthMessage {
    pub signed_timestamp: String,
    pub task_id: u64,
    pub timestamp_signature: Vec<u8>,
    pub public_key: PublicKey,
}

#[derive(Deserialize, Serialize, Debug)]
struct WsAuthResponse {
    response_type: String,
    node_public_key: String,
}

pub fn process_auth_request(request: WsAuthRequest) -> Result<ProcessedAuthMessage, ClientError> {
    // get rid of message prefix if it is there
    let timestamp_signature = if request.signed_timestamp_signature.starts_with("0x") {
        hex::decode(request.signed_timestamp_signature.trim_start_matches("0x"))
            .map_err(|e| ClientError::AuthError(e.to_string()))?
    } else {
        hex::decode(request.signed_timestamp_signature)
            .map_err(|e| ClientError::AuthError(e.to_string()))?
    };

    // convert x25519 public key into the correct format
    let mut array = [0u8; 32];
    println!("{}", request.ephemeral_public_key);
    let public_key_vec = hex::decode(request.ephemeral_public_key)
        .map_err(|e| ClientError::AuthError(e.to_string()))?;

    if public_key_vec.len() != 32 {
        println!("Public key length must be 32 bytes.");
    };

    array.copy_from_slice(&public_key_vec);

    let public_key = PublicKey::from(array);

    // timestamp itself will remain a string, since signature verification and timestamp conversion need it to be different
    // types, so it makes more sense to let the functions do the conversion themselves

    Ok(ProcessedAuthMessage {
        signed_timestamp: request.signed_timestamp,
        timestamp_signature,
        public_key,
        task_id: request.task_id,
    })
}

pub async fn construct_auth_response(
    request: WsAuthRequest,
    diffie_hellman_key: &Arc<RwLock<Option<[u8; 32]>>>,
    log_storage: &Arc<Mutex<Vec<String>>>,
    public_key_bytes: [u8; 32],
) -> Result<String, ClientError> {
    if let Ok(processed_request) = process_auth_request(request) {
        verify_signature(
            processed_request.signed_timestamp,
            &processed_request.timestamp_signature,
            &public_key_bytes,
        )
        .map_err(|e| ClientError::AuthError(e.to_string()))?;

        let server_keypair = generate_server_ephemeral_keypair();

        let server_public_key_bytes = server_keypair.1.to_bytes();

        let mut diffie_hellman_key_guard = diffie_hellman_key.write().await;

        if diffie_hellman_key_guard.is_none() {
            *diffie_hellman_key_guard = Some(compute_diffie_hellman_secret(
                server_keypair.0,
                processed_request.public_key,
            ));
        }

        let current_task = processed_request.task_id;

        let log_storage_clone = Arc::clone(log_storage);

        tokio::spawn(async move {
            // TODO - if this throws an error at any point during the log aggregation it should be visible on the frontend
            if let Err(e) = logs::aggregate_new_logs(log_storage_clone, current_task).await {
                println!("Failed to aggregate logs: {}", e);
            }
        });

        let node_ephemeral_public_key_hex = hex::encode(server_public_key_bytes);

        let message = serde_json::to_string(&WsAuthResponse {response_type: "Auth".to_string(), node_public_key: node_ephemeral_public_key_hex})
            .unwrap_or("Cyborg Agent encoutered an internal error while processing the authorization request, please try again.".to_string());

        println!("Sending auth message: {}", message);

        Ok(message)
    } else {
        Err(ClientError::AuthError(
            "Failed to process auth request.".to_string(),
        ))
    }
}
