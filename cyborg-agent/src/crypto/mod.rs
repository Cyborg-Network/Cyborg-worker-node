use serde_json::from_reader;
use x25519_dalek::{PublicKey, EphemeralSecret};
use rand::rngs::OsRng;
use sp_core::sr25519;
use sp_core::Pair;
use sp_core::ByteArray;
use sp_core::crypto::{AccountId32, Ss58Codec};
use std::{fs::File, io};
use std::time::{SystemTime, UNIX_EPOCH};
use sodiumoxide::crypto::secretbox;
use sodiumoxide::randombytes::randombytes;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct EncryptedMessage {
    response_type: String,
    encrypted_data_hex: String,
    nonce_hex: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AgentConfig {
    pub worker_owner: String,
    pub worker_identity: (String, u64),
    pub task_owner: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskOwner {
    pub task_owner: String,
}

pub fn decode_polkadot_address(address: &str) -> Result<[u8; 32], String> {
    let account_id = AccountId32::from_ss58check(address).map_err(|e| e.to_string())?;
    
    Ok(account_id.into())
}

fn verify_timestamp(received_timestamp: String) -> Result<(), &'static str> {
    let mut stripped_timestamp = received_timestamp;
    // Cleaning up a potential wrapper the polkadot-js wallet added when signing
    if stripped_timestamp.starts_with("<Bytes>") {
        stripped_timestamp = stripped_timestamp.replace("<Bytes>", "").replace("</Bytes>", "");
    }
    let stripped_timestamp = stripped_timestamp
        .parse::<u64>()
        .map_err(|_| "Failed to parse timestamp")?;

    let current_time = SystemTime::now().duration_since(UNIX_EPOCH)
        .map_err(|_| "Failed to get current time")?
        .as_secs();

    let time_window = 15;

    println!("Current time: {}", current_time);
    println!("Received time: {}", stripped_timestamp);

    if stripped_timestamp > current_time || (current_time - stripped_timestamp) > time_window {
        return Err("Timestamp is invalid");
    }

    Ok(())
}  

pub fn verify_signature(signed_timestamp: String, signature_bytes: &[u8], user_polkadot_address_bytes: &[u8]) -> Result<(), &'static str> {
    let user_polkadot_address = sr25519::Public::from_slice(user_polkadot_address_bytes).map_err(|_| "Invalid public key")?;
    
    let signature = sr25519::Signature::from_slice(&signature_bytes).map_err(|_| "Invalid signature")?;

    println!("Signed timestamp: {}", signed_timestamp);

    let timestamp_bytes = signed_timestamp.clone().into_bytes(); 
    
    let is_signature_valid = sr25519::Pair::verify(&signature, &timestamp_bytes, &user_polkadot_address);

    println!("Signature verification result: {}", is_signature_valid);

    let is_timestamp_valid = verify_timestamp(signed_timestamp);

    println!("Timestamp verification result: {}", is_timestamp_valid.is_ok());
    
    if !is_signature_valid {
        println!("Signature verification failed");
        return Err("Signature verification failed");
    } 
    if !is_timestamp_valid.is_ok() {
        print!("Timestamp verification failed");
        return  Err("Timestamp verification failed");
    }

    Ok(())
}

pub fn generate_server_ephemeral_keypair() -> (EphemeralSecret, PublicKey) {
    let server_secret = EphemeralSecret::random_from_rng(OsRng);
    let server_public = PublicKey::from(&server_secret);
    (server_secret, server_public)
}

pub fn compute_diffie_hellman_secret(server_secret: EphemeralSecret, client_public: PublicKey) -> [u8; 32] {
    let shared_secret = server_secret.diffie_hellman(&client_public);
    *shared_secret.as_bytes()
}

pub fn encrypt_message(response_type: &str, diffie_hellman_key: &[u8; 32], data: String) -> EncryptedMessage {
    // Initialize sodiumoxide (should only be called once in your application)
    sodiumoxide::init().unwrap();

    // Generate a nonce (24 bytes for secretbox)
    let nonce = randombytes(secretbox::NONCEBYTES);
    let data_bytes = data.as_bytes();

    // Encrypt the data using secretbox
    let ciphertext = secretbox::seal(
        data_bytes,
        &secretbox::Nonce::from_slice(&nonce).unwrap(),
        &secretbox::Key::from_slice(diffie_hellman_key).unwrap(),
    );

    // Encode the ciphertext and nonce as hex
    let encrypted_data_hex = hex::encode(&ciphertext);
    let nonce_hex = hex::encode(&nonce);

    // Create the final message format
    let message = EncryptedMessage{ response_type: response_type.to_string(), encrypted_data_hex, nonce_hex };

    message
}
 
pub fn read_agent_config() -> Result<AgentConfig, io::Error> {
    let file = File::open("/var/lib/cyborg/worker-node/config/worker_config.json")?;

    let config: AgentConfig = from_reader(file).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    Ok(config)
}

pub fn read_task_owner() -> Result<TaskOwner, io::Error> {
    let file = File::open("/var/lib/cyborg/worker-node/config/task_owner.json")?;

    let config: TaskOwner = from_reader(file).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    Ok(config)
}