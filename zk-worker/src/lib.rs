use std::fs;
use std::path::Path;
use std::process::{Command, Output};

fn run_make_target(target: &str) -> Result<(), Box<dyn std::error::Error>> {
    let status = Command::new("make")
        .arg(target)
        .status()?;

    if status.success() {
        println!("Successfully ran make target: {}", target);
        Ok(())
    } else {
        eprintln!("Make target {} failed to execute", target);
        Err("Make command failed".into())
    }
}

// build circuit and generate witness
pub fn build_circuit_and_witness() -> Result<(), Box<dyn std::error::Error>> { // build_circuit_and_witness
    run_make_target("build")
}

// Initiate powers of tau ceremony for trusted setup
pub fn initiate_powers_of_tau() -> Result<(), Box<dyn std::error::Error>> {
    run_make_target("tau")
}

// Generate proof
pub fn generate_proof() -> Result<(), Box<dyn std::error::Error>> {
    run_make_target("generate-proof")
}

/// Locally verify proof
pub fn verify_proof() -> Result<(), Box<dyn std::error::Error>> {
    run_make_target("verify-proof")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // build_circuit_and_witness().expect("Failed to run make target");
    // initiate_powers_of_tau().expect("Failed to run make tau");
    // generate_proof().expect("Failed to run make tau");
    // verify_proof().expect("Failed to run make tau");
    Ok(())
}

// Test module
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_circuit_and_witness_generation() {
        // Run the make target to create the file
        build_circuit_and_witness().expect("Failed to build circuit and witness files");

        // Check if the files exists
        let task_wasm_path = Path::new("build/task_js/task.wasm");
        assert!(task_wasm_path.exists(), "File task.wasm was not created");

        let witness_path = Path::new("build/task_js/witness.wtns");
        assert!(witness_path.exists(), "File witness.wtns was not created");

        let witness_generator_path = Path::new("build/task_js/witness_calculator.js");
        assert!(witness_generator_path.exists(), "File witness_generator.js was not created");

        let witness_calculator_path = Path::new("build/task_js/witness_calculator.js");
        assert!(witness_calculator_path.exists(), "File witness_calculator.js was not created");

        let task_r1cs_path = Path::new("build/task.r1cs");
        assert!(task_r1cs_path.exists(), "File task.r1cs was not created");

        let task_sym_path = Path::new("build/task.sym");
        assert!(task_sym_path.exists(), "File task.sym was not created");
    }

    #[test]
    #[ignore]
    fn test_powers_of_tau() {
        // Run the make target to create the file
        build_circuit_and_witness().expect("Failed to conduct powers of tau ceremony");

        // Quick check required files
        let wasm_task = Path::new("build/task_js/task.wasm");
        assert!(wasm_task.exists(), "File task.wasm was not created");

        let witness_path = Path::new("build/task_js/witness.wtns");
        assert!(witness_path.exists(), "File witness.wtns was not created");

        let task_r1cs_path = Path::new("build/task.r1cs");
        assert!(task_r1cs_path.exists(), "File task.r1cs was not created");

        // Initiate powers of tau ceremony
        initiate_powers_of_tau().expect("Failed to run make tau");

        // Power of Tau files
        let pot_0 = Path::new("build/pot12_0000.ptau");
        assert!(pot_0.exists(), "File pot12_0000.ptau was not created");

        let pot_1 = Path::new("build/pot12_0001.ptau");
        assert!(pot_1.exists(), "File pot12_0001.ptau was not created");

        let pot_final = Path::new("build/pot12_final.ptau");
        assert!(pot_final.exists(), "File pot12_final.ptau was not created");

        // Task verification key generation files
        let task_0 = Path::new("build/task_0000.zkey");
        assert!(task_0.exists(), "File task_0001.zkey was not created");

        let task_1 = Path::new("build/task_0000.zkey");
        assert!(task_1.exists(), "File task_0001.zkey was not created");

        // Verification key file
        let verification_key_path = Path::new("build/verification_key.json");
        assert!(task_r1cs_path.exists(), "File verification_key.verification_key was not created");
    }

    #[test]
    fn test_generate_proof() {
        // Run the make target to create the file
        generate_proof().expect("Failed to generate proof");

        // Check if the input and proof files exists
        let input = Path::new("build/input.json");
        assert!(input.exists(), "File input.json was not created");

        let proof = Path::new("build/proof.json");
        assert!(proof.exists(), "File proof.json was not created");
    }

    #[test]
    fn test_verifiy_proof() {
        // Verify proof locally
        assert!(verify_proof().is_ok(), "Fail to verify proof");
    }

    #[test]
    fn test_verifiy_proof_fail() {
        assert!(run_make_target("verify-proof-fail").is_err());
    }

    #[test]
    #[ignore]
    fn test_clean_up_files() {
        // Clean up by removing the file after test
        run_make_target("clean-task").expect("Failed to clean up task circuit and witness files");

        let wasm_task = Path::new("build/task_js/task.wasm");
        assert!(!wasm_task.exists());

        let witness_path = Path::new("build/task_js/witness.wtns");
        assert!(!witness_path.exists());
    
        run_make_target("clean-pot").expect("Failed to clean up pot files");

        let verification_key_path = Path::new("build/verification_key.json");
        assert!(!verification_key_path.exists());

        run_make_target("clean-proof").expect("Failed to clean up proof files");
        
        let proof = Path::new("build/proof.json");
        assert!(!proof.exists());
    }
}

