use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::env;
use std::process::Stdio;
use std::io::Write;
use rand::Rng;
use std::io::BufReader;
use std::thread;
use std::io::{BufRead, stdin, stdout};


pub fn set_environment() {
    let _current_file = file!();
    let current_dir = Path::new("/var/lib/cyborg/worker-node/zk-files");
    let current_dir = Path::new(current_dir)
        .parent()
        .and_then(|p| p.parent())
        .expect("Failed to get the parent of the parent directory")
        .canonicalize()
        .expect("Failed to canonicalize path");

    env::set_current_dir(&current_dir).expect("Failed to change directory");
}

fn run_make_target(target: &str) -> Result<(), Box<dyn std::error::Error>> {
    set_environment();
    let output = Command::new("make")
    .arg(target)
    .output()
    .expect("Failed to execute command");

    if output.status.success() {
        println!("Successfully ran make target: {}", target);
        Ok(())
    } else {
        eprintln!("Make target {} failed to execute", target);
        eprintln!("Command failed with exit code {}.", output.status.code().unwrap());
        eprintln!("Error details: {}", String::from_utf8_lossy(&output.stderr));
        Err("Make command failed".into())
    }
}

// build circuit and generate witness
pub fn build_circuit_and_witness() -> Result<(), Box<dyn std::error::Error>> { // build_circuit_and_witness
    run_make_target("build-step")
}

// Initiate powers of tau ceremony for trusted setup
pub fn initiate_powers_of_tau() -> Result<(), Box<dyn std::error::Error>> {
    set_environment();

    let mut rng = rand::thread_rng();

    let mut child = Command::new("make")
        .arg("tau")
        .stdin(Stdio::piped())  
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start process");

        let mut stdin = child.stdin.take().expect("Failed to capture stdin");
        let stdout = child.stdout.take().expect("Failed to capture stdout");
        let reader = BufReader::new(stdout);
        
        for line in reader.lines() {
            let line = line.expect("Failed to read line");
            println!("Output from command: {}", line);
    
            let entropy: u32 = rng.gen_range(1..=1000); 
            writeln!(stdin, "{}", entropy).expect("Failed to write to stdin");
        }

    let status = child.wait().expect("Failed to wait on child");

    if status.success() {
        println!("Command executed successfully.");
        Ok(())
    } else {
        eprintln!("Command failed.");
        Err("Tau trusted setup failed".into())
    }
}

// Generate proof
pub fn generate_proof() -> Result<(), Box<dyn std::error::Error>> {
    run_make_target("generate-proof")
}

/// Locally verify proof
pub fn verify_proof() -> Result<(), Box<dyn std::error::Error>> {
    run_make_target("verify-proof")
}

pub fn clean_up_old_zk_files() {
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


fn main() -> Result<(), Box<dyn std::error::Error>> {
    build_circuit_and_witness().expect("Failed to run make target");
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

