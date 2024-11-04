use std::process::{Command, Output};

/// Runs a specific make target and returns the output
pub fn run_make_task(task: &str) -> Output {
    Command::new("make")
        .arg(task)
        .output()
        .expect(&format!("Failed to execute make target: {}", task))
}

/// Calls the `make build` command
pub fn make_build() {
    let output = run_make_task("build");
    if output.status.success() {
        println!("Build successful:\n{}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprintln!("Build failed:\n{}", String::from_utf8_lossy(&output.stderr));
    }
}

/// Calls the `make tau` command
pub fn make_tau() {
    let output = run_make_task("tau");
    if output.status.success() {
        println!("Powers of Tau ceremony completed:\n{}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprintln!("Powers of Tau ceremony failed:\n{}", String::from_utf8_lossy(&output.stderr));
    }
}

/// Calls the `make generate-proof` command
pub fn make_generate_proof() {
    let output = run_make_task("generate-proof");
    if output.status.success() {
        println!("Proof generated successfully:\n{}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprintln!("Proof generation failed:\n{}", String::from_utf8_lossy(&output.stderr));
    }
}

/// Calls the `make verify-proof` command
pub fn make_verify_proof() {
    let output = run_make_task("verify-proof");
    if output.status.success() {
        println!("Proof verified successfully:\n{}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprintln!("Proof verification failed:\n{}", String::from_utf8_lossy(&output.stderr));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_build() {
        assert!(make_build(), "make build failed");
    }

    #[test]
    fn test_make_tau() {
        assert!(make_tau(), "make tau failed");
    }

    #[test]
    fn test_make_generate_proof() {
        assert!(make_generate_proof(), "make generate-proof failed");
    }

    #[test]
    fn test_make_verify_proof() {
        assert!(make_verify_proof(), "make verify-proof failed");
    }
}
