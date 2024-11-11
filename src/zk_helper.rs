use zk_worker::{ build_circuit_and_witness, initiate_powers_of_tau, clean_up_old_zk_files};
use std::fs;
use std::io;
use std::path::PathBuf;
use subxt::{OnlineClient, PolkadotConfig};
use subxt_signer::sr25519::Keypair;

use crate::substrate_interface;
use substrate_interface::api::runtime_types::bounded_collections::bounded_vec::BoundedVec;

use std::env;
use std::path::Path;

pub async fn fetch_and_build() {
    //let original_dir = PathBuf::from("/usr/local/bin");
    //migrate_circuit_and_input_to_zk_worker();
    build_circuit_and_witness();
    //env::set_current_dir(&original_dir).expect("Failed to change directory");
}

pub async fn generate_trusted_setup() {
    //let original_dir = PathBuf::from("/usr/local/bin");
    initiate_powers_of_tau();
    //env::set_current_dir(&original_dir).expect("Failed to change directory");
}

fn migrate_circuit_and_input_to_zk_worker() -> io::Result<()> {
    let circuit_src = "/var/lib/cyborg/worker-node/zk-files/zk_circuit.circom";
    let input_src = "/var/lib/cyborg/worker-node/zk-files/zk_public_input.json";
    let input_dest = "zk-worker/input.json";
    let circuit_dest = "zk-worker/task.circom";

    match fs::copy(circuit_src, circuit_dest) {
        Ok(_) => {
            println!("Circuit files copied and replaced successfully!");
        }
        Err(e) => {
            eprintln!("Failed to copy circuit file: {}", e);
            return Err(e);
        }
    }

    match fs::copy(input_src, input_dest) {
        Ok(_) => {
            println!("Input files copied and replaced successfully!");
        }
        Err(e) => {
            eprintln!("Failed to copy Input file: {}", e);
            return Err(e);
        }
    }
    Ok(())
}


pub async fn submit_trusted_setup_onchain(api: &OnlineClient<PolkadotConfig>, signer_keypair: &Keypair, task_id: u64)
    -> Result<(), Box<dyn std::error::Error>> 
{
    let path = "/var/lib/cyborg/worker-node/zk-files/build/verification_key.json";
    let path2 = "/var/lib/cyborg/worker-node/zk-files/input.json";

    let json_verification_key = fs::read_to_string(path)?;
    let json_pub_input = fs::read_to_string(path2)?;

    let bytes_verification_key: Vec<u8> = json_verification_key.into_bytes();
    let bytes_pub_input: Vec<u8> = json_pub_input.into_bytes();

    let result_trusted_setup_tx = substrate_interface::api::tx()
        .zk_verifier()
        .setup_verification(
            task_id, 
            bytes_pub_input, 
            bytes_verification_key,
        );

    println!("Transaction Details:");
    println!("Module: {:?}", result_trusted_setup_tx.pallet_name());
    println!("Call: {:?}", result_trusted_setup_tx.call_name());
    println!("Parameters: {:?}", result_trusted_setup_tx.call_data());

    let result_trusted_setup_events= api
        .tx()
        .sign_and_submit_then_watch_default(&result_trusted_setup_tx, signer_keypair)
        .await
        .map(|e| {
            println!("Verification key added, waiting for transaction to be finalized...");
            e
        })?
        .wait_for_finalized_success()
        .await?;

    let trusted_setup_verification_event = 
        result_trusted_setup_events.find_first::<substrate_interface::api::zk_verifier::events::VerificationSetupCompleted>()?;
    if let Some(event) = trusted_setup_verification_event {
        println!("Setup completed successfully: {event:?}");
    } else {
        println!("Setup completed failed");
    }

    Ok(())
}


pub async fn verify_zk_onchain(api: &OnlineClient<PolkadotConfig>, signer_keypair: &Keypair, task_id: u64)
    -> Result<(), Box<dyn std::error::Error>> 
{
    let path = "/var/lib/cyborg/worker-node/zk-files/build/proof.json";

    let json_proof = fs::read_to_string(path)?;

    let bytes_proof: Vec<u8> = json_proof.into_bytes();

    let result_verify_tx = substrate_interface::api::tx()
        .zk_verifier()
        .verify(
            task_id, 
            bytes_proof,
        );

    println!("Transaction Details:");
    println!("Module: {:?}", result_verify_tx.pallet_name());
    println!("Call: {:?}", result_verify_tx.call_name());
    println!("Parameters: {:?}", result_verify_tx.call_data());

    let result_verify_events= api
        .tx()
        .sign_and_submit_then_watch_default(&result_verify_tx, signer_keypair)
        .await
        .map(|e| {
            println!("Verification key added, waiting for transaction to be finalized...");
            e
        })?
        .wait_for_finalized_success()
        .await?;

    let trusted_verify_event = 
        result_verify_events.find_first::<substrate_interface::api::zk_verifier::events::VerificationSuccess>()?;
    if let Some(event) = trusted_verify_event {
        println!("ZK proof successfully verified: {event:?}");
    } else {
        println!("Verifying ZK proof failed");
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;


    #[test]
    fn test_migrate_circuit_and_input_to_zk_worker() {
        migrate_circuit_and_input_to_zk_worker();

        let circuit_src = "zk_circuit.circom";
        let input_src = "zk_public_input.json";

        let input_dest = "zk-worker/input.json";
        let circuit_dest = "zk-worker/task.circom";

        let original_input_file = fs::read(input_src).unwrap();
        let migrated_input_file = fs::read(input_dest).unwrap();
        assert!(original_input_file == migrated_input_file);

        let original_circuit_file = fs::read(circuit_src).unwrap();
        let migrated_circuit_file = fs::read(circuit_dest).unwrap();

        assert!(original_circuit_file == migrated_circuit_file);
    }

    #[test]
    fn test_zk_setup_files_fetchable() {
        let json_verification_key = fs::read_to_string("../zk-worker/build/verification_key.json").unwrap();
        let json_pub_input = fs::read_to_string("../zk-worker/input.json").unwrap();

        let bytes_verification_key: Vec<u8> = json_verification_key.into_bytes();
        let bytes_pub_input: Vec<u8> = json_pub_input.into_bytes();

        assert!(bytes_verification_key.len()>0);
        assert!(bytes_pub_input.len()>0);
    }

    #[test]
    fn test_zk_proof_files_fetchable() {
        let json_proof = fs::read_to_string("../zk-worker/build/proof.json").unwrap();

        let bytes_proof: Vec<u8> = json_proof.into_bytes();

        assert!(bytes_proof.len()>0);
    }
}
