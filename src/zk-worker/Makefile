# Makefile

# Default rule
all: build tau generate-proof verify-proof

# Performs build step
build: compile-circuit compute-witness

# Go to build directory
go-to-build:
	cd build

# Compile circuit
compile-circuit:
	circom task.circom --r1cs --wasm --sym -o build --O0 -p bls12381

# Compute witness
compute-witness:
	cd build/task_js && node generate_witness.js task.wasm ../../input.json witness.wtns

# Performs Powers of Tau step
tau: tau-create-ceremony tau-first-contribution tau-phase-2 tau-z-key tau-second-contribution tau-export-vk

# Create ceremony
tau-create-ceremony:
	cd build && snarkjs powersoftau new bls12381 12 pot12_0000.ptau -v

# Make first contribution
tau-first-contribution:
	cd build && snarkjs powersoftau contribute pot12_0000.ptau pot12_0001.ptau --name="ZkSnarks phase #1" -v

# Start phase 2
tau-phase-2:
	cd build && snarkjs powersoftau prepare phase2 pot12_0001.ptau pot12_final.ptau -v

# Generate z-key
tau-z-key:
	cd build && snarkjs groth16 setup task.r1cs pot12_final.ptau task_0000.zkey

# Make second contribution
tau-second-contribution:
	cd build && snarkjs zkey contribute task_0000.zkey task_0001.zkey --name="ZkSnarks phase #2" -v

# Export verification-key
tau-export-vk:
	cd build && snarkjs zkey export verificationkey task_0001.zkey verification_key.json -v

# Generate proof
generate-proof:
	cd build && snarkjs groth16 prove task_0001.zkey task_js/witness.wtns proof.json input.json

# Verify proof
verify-proof:
	cd build && snarkjs groth16 verify verification_key.json input.json proof.json
