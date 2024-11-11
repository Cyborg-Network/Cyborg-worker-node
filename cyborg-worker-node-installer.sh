#!/bin/bash

set -e

trap 'echo "An error occurred during installation, please check the logs for further information."; exit -1' ERR

HTTP_PORT=8080
WS_PORT=8081

AGENT_BINARY_URL="server.cyborgnetwork.io:8080/assets/cyborg-agent"
AGENT_BINARY_NAME="cyborg-agent"
AGENT_BINARY_PATH="/usr/local/bin/$AGENT_BINARY_NAME"
AGENT_SERVICE_FILE="/etc/systemd/system/$AGENT_BINARY_NAME.service"

WORKER_BINARY_URL="server.cyborgnetwork.io:8080/assets/cyborg-worker-node"
WORKER_BINARY_NAME="cyborg-worker-node"
WORKER_BINARY_PATH="/usr/local/bin/$WORKER_BINARY_NAME"
WORKER_SERVICE_FILE="/etc/systemd/system/$WORKER_BINARY_NAME.service"
WORKER_DBUS_FILE="/etc/dbus-1/system.d/com.cyborg.CyborgAgent.conf"
WORKER_MAKEFILE="/var/lib/cyborg/worker-node/zk-files/Makefile"

echo "Installing cargo..."

cargo install --git https://github.com/iden3/circom 

echo "Installing npm..."

npm install -g snarkjs


echo "Downloading the worker node from $WORKER_BINARY_URL..."
echo "Downloading the agent from $AGENT_BINARY_URL..."
curl -L -o $WORKER_BINARY_NAME $WORKER_BINARY_URL
curl -L -o $AGENT_BINARY_NAME $AGENT_BINARY_URL

chmod +x $WORKER_BINARY_NAME
chmod +x $AGENT_BINARY_NAME

echo "Moving the worker to /usr/local/bin..."
echo "Moving the agent to /usr/local/bin..."

sudo mv $WORKER_BINARY_NAME $WORKER_BINARY_PATH
sudo mv $AGENT_BINARY_NAME $AGENT_BINARY_PATH

echo "Initiating worker registration..."

read -p "Please provide an endpoint to the parachain that the worker will be registered on: " PARACHAIN_URL

read -p "Please enter the seed phrase of the account that will be managing the worker node: " ACCOUNT_SEED

read -p "Please enter the URL of the IPFS API you want to use: " IPFS_URL

read -p "Please enter the API key of the IPFS API you want to use: " IPFS_KEY

read -p "Please enter the API secret of the IPFS API you want to use: " IPFS_SECRET

export CYBORG_WORKER_NODE_IPFS_API_URL="$IPFS_URL"
export CYBORG_WORKER_NODE_IPFS_API_KEY="$IPFS_KEY"
export CYBORG_WORKER_NODE_IPFS_API_SECRET="$IPFS_SECRET"

echo $CYBORG_WORKER_NODE_IPFS_API_URL
echo $CYBORG_WORKER_NODE_IPFS_API_KEY
echo $CYBORG_WORKER_NODE_IPFS_API_SECRET

sed -i '/CYBORG_WORKER_NODE_IPFS_API_URL/d' ~/.bashrc
sed -i '/CYBORG_WORKER_NODE_IPFS_API_KEY/d' ~/.bashrc
sed -i '/CYBORG_WORKER_NODE_IPFS_API_SECRET/d' ~/.bashrc

echo "export CYBORG_WORKER_NODE_IPFS_API_URL=\"$IPFS_URL\"" >> ~/.bashrc
echo "export CYBORG_WORKER_NODE_IPFS_API_KEY=\"$IPFS_KEY\"" >> ~/.bashrc
echo "export CYBORG_WORKER_NODE_IPFS_API_SECRET=\"$IPFS_SECRET\"" >> ~/.bashrc

source ~/.bashrc

if ! id "cyborg-user" &>/dev/null; then
    sudo useradd -r -s /bin/false cyborg-user
fi

sudo mkdir -p /var/lib/cyborg/worker-node/packages
sudo mkdir -p /var/lib/cyborg/worker-node/config
sudo mkdir -p /var/lib/cyborg/worker-node/logs
sudo mkdir -p /var/lib/cyborg/worker-node/zk-files
sudo chown -R cyborg-user:cyborg-user /var/lib/cyborg
sudo chmod -R 700 /var/lib/cyborg

sudo $WORKER_BINARY_PATH registration --parachain-url "$PARACHAIN_URL" --account-seed "$ACCOUNT_SEED"

echo "Creating dbus configuration file for worker node..."
sudo bash -c "cat > $WORKER_DBUS_FILE" << EOL
<!DOCTYPE busconfig PUBLIC
 "-//freedesktop//DTD D-BUS Bus Configuration 1.0//EN"
 "http://www.freedesktop.org/standards/dbus/1.0/busconfig.dtd">
<busconfig>

  <policy context="default">
    <allow own="com.cyborg.CyborgAgent"/>
  </policy>

  <policy context="default">
    <allow send_interface="com.cyborg.AgentZkInterface"/>
  </policy>

</busconfig>
EOL

echo "Creating systemd service for worker node: $WORKER_SERVICE_FILE"
sudo bash -c "cat > $WORKER_SERVICE_FILE" << EOL
[Unit]
Description=Binary that will execute compute requests from the cyborg-parachain.
After=network.target

[Service]
User=cyborg-user
Group=cyborg-user
Environment=PARACHAIN_URL=$PARACHAIN_URL
Environment="ACCOUNT_SEED=\"$ACCOUNT_SEED\""
Environment="CYBORG_WORKER_NODE_IPFS_API_URL=$IPFS_URL"
Environment="CYBORG_WORKER_NODE_IPFS_API_KEY=$IPFS_KEY"
Environment="CYBORG_WORKER_NODE_IPFS_API_SECRET=$IPFS_SECRET"
ExecStart=$WORKER_BINARY_PATH startmining --parachain-url \$PARACHAIN_URL --account-seed "\$ACCOUNT_SEED"
Restart=always
RestartSec=3

[Install]
WantedBy=multi-user.target
EOL

echo "Worker node service created successfully!"

echo "Creating zk Makefile..."
sudo bash -c "cat > $WORKER_ZK_MAKEFILE" << EOL
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

# Verify proof
verify-proof-fail:
	cd build && snarkjs groth16 verify verification_key.json input.json dummy_proof.json

# Clean up the generated file for task
clean-task:
	rm -rf build/task_js
	rm -f build/task.r1cs
	rm -f build/task.sym

# Clean up powers of tau trusted setup files
clean-pot:
	rm -f build/*.ptau
	rm -f build/*.zkey
	rm -f build/verification_key.json

# Clean up proof and input files
clean-proof:
	rm -f build/input.json
	rm -f build/proof.json


clean-all: clean-task clean-pot clean-proof

test: cargo test -- --test-threads=1
EOL

echo "ZK Makefile created successfully!"

echo "Installing ZK dependencies..."

sudo make -C $WORKER_ZK_MAKEFILE install-zk-deps

echo "Creating systemd service for agent: $AGENT_SERVICE_FILE"
sudo bash -c "cat > $AGENT_SERVICE_FILE" << EOL
[Unit]
Description=Agent that is able to check the health of the node, provide reuired info to the cyborg-parachain, and stream usage metrics and logs of the cyborg node.
After=network.target

[Service]
User=cyborg-user
Group=cyborg-user
ExecStart=$AGENT_BINARY_PATH run
Restart=always
RestartSec=3

[Install]
WantedBy=multi-user.target
EOL

echo "Agent service created successfully!"

echo "Reloading systemd, enabling and starting $WORKER_BINARY_NAME and $AGENT_BINARY_NAME services..."
sudo systemctl daemon-reload
sudo systemctl enable $WORKER_BINARY_NAME
sudo systemctl enable $AGENT_BINARY_NAME
sudo systemctl start $WORKER_BINARY_NAME
sudo systemctl start $AGENT_BINARY_NAME

sudo systemctl status $WORKER_BINARY_NAME --no-pager
sudo systemctl status $AGENT_BINARY_NAME --no-pager


echo "Cyborg Worker Node and Agent are installed and running. Binaries are located at $WORKER_BINARY_PATH and $AGENT_BINARY_PATH. Now attempting to open Port $HTTP_PORT and $WS_PORT to enable communication with Cyborg Connect."

if command -v ufw &> /dev/null; then
    FIREWALL="ufw"
elif command -v firewall-cmd &> /dev/null; then
    FIREWALL="firewalld"
elif command -v iptables &> /dev/null; then
    FIREWALL="iptables"
else
    echo "Firewall management tool not detected. Please open $HTTP_PORT and $WS_PORT manually for the agent to work."
    echo "If in doubt, refer to the documentation of your firewall management tool for instructions."
fi

open_ports_ufw() {
    sudo ufw allow $WS_PORT
    sudo ufw allow $HTTP_PORT
    echo "Ports opened in UFW."
}

# Function to open ports with firewalld
open_ports_firewalld() {
    sudo firewall-cmd --permanent --add-port=$HTTP_PORT/tcp
    sudo firewall-cmd --permanent --add-port=$WS_PORT/tcp
    sudo firewall-cmd --reload
    echo "Ports opened in firewalld."
}

# Function to open ports with iptables
open_ports_iptables() {
    sudo iptables -A INPUT -p tcp --dport $HTTP_PORT -j ACCEPT
    sudo iptables -A INPUT -p tcp --dport $WS_PORT -j ACCEPT
    # Note: Rules added with iptables are not persistent across reboots unless saved.
    echo "Ports opened in iptables."
}

case $FIREWALL in
    "ufw")
        open_ports_ufw
        ;;
    "firewalld")
        open_ports_firewalld
        ;;
    "iptables")
        open_ports_iptables
        ;;
esac
