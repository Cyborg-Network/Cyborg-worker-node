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

CIRCOM_URL="server.cyborgnetwork.io:8080/assets/circom"

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

if [ -f "$HOME/.cargo/env" ]; then
    . "$HOME/.cargo/env"
fi

check_command() {
    command -v "$1" &> /dev/null
}

cd ~

if check_command cargo; then
    cargo_version=$(cargo --version)
    echo "Cargo is installed. Version: $cargo_version. Proceeding..."
else
    echo "Cargo isn't installed. Installing via rustup..."
    curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
fi

. "$HOME/.cargo/env"

if check_command npm; then
    npm_version=$(npm --version)
    echo "Npm is installed. Version: $npm_version. Proceeding..."
else
    echo "Npm isn't installed. Installing via nodesource..."    
    curl -fsSL https://deb.nodesource.com/setup_23.x -o /tmp/nodesource_setup.sh
    sudo sudo bash /tmp/nodesource_setup.sh

    sudo apt-get install nodejs -y

    sudo rm /tmp/nodesource_setup.sh
fi

echo "Would you like to install Circom via:"
echo "1) Download (using precompiled binary, requires less computation power)"
echo "2) Compile locally (requires more computation power)"
read -p "Please choose an option (1 or 2): " choice

download_circom() {
    echo "Downloading precompiled Circom binary..."

    sudo git clone https://github.com/iden3/circom.git

    mkdir -p circom/target/release

    cd circom/target/release

    curl -L -o circom $CIRCOM_URL
    sudo chmod +x circom

    cd ../../

    cargo install --path circom

    echo "Circom downloaded and ready to use!"
}

compile_circom_locally() {
    echo "Compiling Circom locally..."

    sudo git clone https://github.com/iden3/circom.git
    cd circom

    cargo build --release

    cargo install --path circom

    echo "Circom compiled and ready to use!"
}

sudo rm -rf circom

# Run according to user choice
if [ "$choice" -eq 1 ]; then
    download_circom
elif [ "$choice" -eq 2 ]; then
    compile_circom_locally
else
    echo "Invalid option. Please choose either 1 or 2."
    exit 1
fi

cd ~

echo "Installing snarkjs..."

sudo npm install -g snarkjs

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
sudo mkdir -p /var/lib/cyborg/worker-node/zk-worker-build
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
