
# Run the complete installation (Circom + snarkjs)
install-zk-deps: install-circom install-snarkjs
	@echo "Circom and snarkjs have been successfully installed."

# Install Circom using cargo
install-circom:
	@echo "Installing Circom..."
	cargo install --git https://github.com/iden3/circom 

# Install snarkjs using npm
install-snarkjs:
	@echo "Installing snarkjs..."
	npm install -g snarkjs

# Default rule
all: build-step tau generate-proof verify-proof

# Performs build step
build-step: compile-circuit compute-witness

# Go to build directory
go-to-build:
	cd /var/lib/cyborg/worker-node/zk-files/build

# Compile circuit
compile-circuit:
	circom task.circom --r1cs --wasm --sym -o /var/lib/cyborg/worker-node/zk-files/build --O0 -p bls12381

# Compute witness
compute-witness:
	cd /var/lib/cyborg/worker-node/zk-files/build/task_js && node generate_witness.js task.wasm ../../input.json witness.wtns

# Performs Powers of Tau step
tau: tau-create-ceremony tau-first-contribution tau-phase-2 tau-z-key tau-second-contribution tau-export-vk

# Create ceremony
tau-create-ceremony:
	cd /var/lib/cyborg/worker-node/zk-files/build && snarkjs powersoftau new bls12381 12 pot12_0000.ptau -v

# Make first contribution
tau-first-contribution:
	cd /var/lib/cyborg/worker-node/zk-files/build && snarkjs powersoftau contribute pot12_0000.ptau pot12_0001.ptau --name="ZkSnarks phase #1" -v

# Start phase 2
tau-phase-2:
	cd /var/lib/cyborg/worker-node/zk-files/build && snarkjs powersoftau prepare phase2 pot12_0001.ptau pot12_final.ptau -v

# Generate z-key
tau-z-key:
	cd /var/lib/cyborg/worker-node/zk-files/build && snarkjs groth16 setup task.r1cs pot12_final.ptau task_0000.zkey

# Make second contribution
tau-second-contribution:
	cd /var/lib/cyborg/worker-node/zk-files/build && snarkjs zkey contribute task_0000.zkey task_0001.zkey --name="ZkSnarks phase #2" -v

# Export verification-key
tau-export-vk:
	cd /var/lib/cyborg/worker-node/zk-files/build && snarkjs zkey export verificationkey task_0001.zkey verification_key.json -v

# Generate proof
generate-proof:
	cd /var/lib/cyborg/worker-node/zk-files/build && snarkjs groth16 prove task_0001.zkey task_js/witness.wtns proof.json input.json

# Verify proof
verify-proof:
	cd /var/lib/cyborg/worker-node/zk-files/build && snarkjs groth16 verify verification_key.json input.json proof.json

# Verify proof
verify-proof-fail:
	cd /var/lib/cyborg/worker-node/zk-files/build && snarkjs groth16 verify verification_key.json input.json dummy_proof.json

# Clean up the generated file for task
clean-task:
	rm -rf /var/lib/cyborg/worker-node/zk-files/build/task_js
	rm -f /var/lib/cyborg/worker-node/zk-files/build/task.r1cs
	rm -f /var/lib/cyborg/worker-node/zk-files/build/task.sym

# Clean up powers of tau trusted setup files
clean-pot:
	rm -f /var/lib/cyborg/worker-node/zk-files/build/*.ptau
	rm -f /var/lib/cyborg/worker-node/zk-files/build/*.zkey
	rm -f /var/lib/cyborg/worker-node/zk-files/build/verification_key.json

# Clean up proof and input files
clean-proof:
	rm -f /var/lib/cyborg/worker-node/zk-files/build/input.json
	rm -f /var/lib/cyborg/worker-node/zk-files/build/proof.json


clean-all: clean-task clean-pot clean-proof

test: cargo test -- --test-threads=1
