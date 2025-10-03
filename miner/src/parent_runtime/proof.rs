use std::path::PathBuf;

use crate::{error::{Error, Result}, global_config::PATHS};
use neuro_zk_runtime::{self, NeuroZKEngine};

pub async fn generate_proof() -> Result<Vec<u8>> {
    let engine = NeuroZKEngine::new(
        PathBuf::from(format!("{}/{}", &PATHS.task_dir_path, &PATHS.task_file_name))
        ).map_err(
            |e| Error::Custom(format!("Failed to create engine: {}", e.to_string()))
        )?;

    let proof = engine.prove_inference(
        &PATHS.task_dir_path, 
        "network.ezkl", 
        "pk.key", 
        "kzg.srs", 
        "proof-witness.json", 
        "input.json"
    )
    .await
    .map_err(|e| Error::Custom(format!("Failed to generate proof: {}", e.to_string())))?;

    Ok(proof.into())
}
