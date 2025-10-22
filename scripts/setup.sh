#!/usr/bin/env bash
set -euo pipefail

if [[ $EUID -ne 0 ]]; then
   echo "This script must be run as root" 
   exit 1
fi

# =================================== SHARED CONFIG ==========================================
REPO="Cyborg-Network/Cyborg-miner"
MINER_ASSET_NAME="cyborg-miner"
PLATFORM="linux"

case "$(uname -m)" in
    x86_64)
        ARCH="x86_64"
        ;;
    aarch64 | arm64)
        ARCH="aarch64"
        ;;
    *)
        echo "Unsupported architecture: $(uname -m)"
        exit 1
        ;;
esac

TAG=$(curl -s https://api.github.com/repos/${REPO}/releases/latest | grep -Po '"tag_name": "\K.*?(?=")')
ASSET="${MINER_ASSET_NAME}-${PLATFORM}-${ARCH}.tar.gz"
URL="https://github.com/${REPO}/releases/download/${TAG}/${ASSET}"

echo "Detected architecture: ${ARCH}"
echo "Downloading release asset: ${ASSET}"

# File names as they appear after installation
MINER_FILE_NAME="cyborg-miner"
AGENT_FILE_NAME="cyborg-agent"
SETUP_SCRIPT_FILE_NAME="setup.sh"

# Paths for the files
BIN_DIR="/usr/local/bin"
SCRIPT_DIR="/var/lib/cyborg/miner/scripts"

# Full paths
MINER_BINARY_PATH="$BIN_DIR/$MINER_FILE_NAME"
AGENT_BINARY_PATH="$BIN_DIR/$AGENT_FILE_NAME"
SETUP_SCRIPT_PATH="$SCRIPT_DIR/$SETUP_SCRIPT_FILE_NAME"

# Ports to be opened at the end of the script
MINER_INFERENCE_PORT=3000
AGENT_HTTP_PORT=8080
AGENT_WS_PORT=8081
FLASH_INFER_PORT=3005

# Service files
MINER_SERVICE_FILE="/etc/systemd/system/$MINER_FILE_NAME.service"
AGENT_SERVICE_FILE="/etc/systemd/system/$AGENT_FILE_NAME.service"

# ENV variables for the miner
MINER_TASK_DIR="/var/lib/cyborg/miner/task"
MINER_CONFIG_DIR="/etc/cyborg/miner"
MINER_LOG_DIR="/var/log/cyborg/miner"

# The tailscale network (only for testnet) on which the miner will be reachable
TAILSCALE_NET="tail78ea2b.ts.net"

verify_release() {
    #local file="$1"
    #local sig_file="${file}.sig"
    
    #curl -L "${URL}.sig" -o "$sig_file"
    
    #if ! minisign -Vm "$file" -P "<YOUR_PUBLIC_KEY>"; then
        #echo "SIGNATURE VERIFICATION FAILED!"
        #exit 1
    #fi
    
    echo "CRITICAL WARNING: Signature verification is not implemented yet!"
}

# ======================================= UTIL ===============================================================
download_and_extract() {
    verify_release

    TMP_DIR=$(mktemp -d)
    trap "rm -rf \"$TMP_DIR\"" EXIT

    echo "Downloading latest release: $TAG..."
    curl -L "$URL" -o "$TMP_DIR/release.tar.gz"
    tar -xf "$TMP_DIR/release.tar.gz" -C "$TMP_DIR"
    
    MINER_BIN=$(find "$TMP_DIR" -type f -executable -name '*miner*' | head -n 1)
    AGENT_BIN=$(find "$TMP_DIR" -type f -executable -name '*agent*' | head -n 1)
    SETUP_SCRIPT=$(find "$TMP_DIR" -type f -executable -name '*setup*' | head -n 1)

    if [[ -z "$MINER_BIN" || -z "$AGENT_BIN" || -z "$SETUP_SCRIPT" ]]; then
        echo "Required files not found."
        exit 1
    fi 

    chmod +x "$MINER_BIN" "$AGENT_BIN" "$SETUP_SCRIPT"
}

prepare_environment() {
    echo "Preparing file system structure..."

    for dir in \
        "$BIN_DIR" \
        "$SCRIPT_DIR" \
        "$MINER_TASK_DIR" \
        "$MINER_CONFIG_DIR" \
        "$MINER_LOG_DIR" \
        "/var/log/cyborg/agent" \
        "/var/lib/cyborg" \
        "/var/log/cyborg" \
        "/etc/cyborg"
    do
        if [[ ! -d "$dir" ]]; then
            echo "Creating directory: $dir"
            mkdir -p "$dir"
        fi
    done

    echo "Setting ownership and permissions..."
    chown -R root:root /var/lib/cyborg /var/log/cyborg /etc/cyborg
    chmod -R 755 /var/lib/cyborg /var/log/cyborg /etc/cyborg
}

setup_docker() {
    if ! command -v docker &> /dev/null; then
        echo "[!] Docker not found. Installing Docker..."
        apt-get update
        apt-get install -y apt-transport-https ca-certificates curl gnupg lsb-release

        curl -fsSL https://download.docker.com/linux/ubuntu/gpg | gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg

        echo \
        "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] \
        https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable" | \
        tee /etc/apt/sources.list.d/docker.list > /dev/null

        apt-get update
        apt-get install -y docker-ce docker-ce-cli containerd.io
        echo "Docker installed successfully."
    else
        echo "Docker is already installed."
    fi

    echo "Docker setup complete."
}

prepare_triton() {
    echo "[*] Triton model repository directory: $MINER_TASK_DIR"

    if [ ! -d "$MINER_TASK_DIR" ]; then
        echo "[!] Triton Model repository folder '$MINER_TASK_DIR' does not exist. Creating it..."
        mkdir -p "$MINER_TASK_DIR"
        echo "[✓] Created empty model directory."
    fi

    if ! command -v docker &> /dev/null; then
        echo "[!] Docker is not installed. Installing Docker..."
        apt-get update
        apt-get install -y apt-transport-https ca-certificates curl gnupg lsb-release

        curl -fsSL https://download.docker.com/linux/ubuntu/gpg | gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg

        echo \
        "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] \
        https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable" | \
        tee /etc/apt/sources.list.d/docker.list > /dev/null

        apt-get update
        apt-get install -y docker-ce docker-ce-cli containerd.io
        echo "[✓] Docker installed."
    else
        echo "[✓] Docker is already installed."
    fi

    TRITON_IMAGE="nvcr.io/nvidia/tritonserver:25.06-py3"
    TRITON_CONTAINER_NAME="triton_server"

    if docker ps -a --format '{{.Names}}' | grep -q "^$TRITON_CONTAINER_NAME\$"; then
        if docker inspect -f '{{.State.Running}}' "$TRITON_CONTAINER_NAME" | grep -q "true"; then
            echo "[✓] Triton container '$TRITON_CONTAINER_NAME' is already running."
        else
            echo "[~] Triton container exists but is not running. Restarting..."
            docker start "$TRITON_CONTAINER_NAME"
        fi
    else
        echo "[*] Pulling Triton server image..."
        docker pull "$TRITON_IMAGE"

        echo "[🚀] Starting Triton server..."
        docker run -d --name "$TRITON_CONTAINER_NAME" --restart unless-stopped \
            -p8000:8000 -p8001:8001 -p8002:8002 \
            -v "$MINER_TASK_DIR":/models \
            "$TRITON_IMAGE" \
            tritonserver --model-repository=/models --model-control-mode=explicit
    fi
}

setup_systemd() {
    local PARACHAIN_URL="$1"
    local ACCOUNT_SEED="$2"
    local MINER_TYPE="$3"

    echo "Creating systemd service for worker node: $MINER_SERVICE_FILE"

    bash -c "cat > $MINER_SERVICE_FILE" << EOL
    [Unit]
    Description=Service running the cyborg-miner.
    After=network.target
    Requires=docker.service

    [Service]
    Type=simple
    User=root
    SupplementaryGroups=docker
    Environment=PARACHAIN_URL=$PARACHAIN_URL
    Environment="ACCOUNT_SEED=\"$ACCOUNT_SEED\""
    Environment=LOG_FILE_PATH=$MINER_LOG_DIR/miner.log
    Environment=UPDATER_LOG_FILE_PATH=$MINER_LOG_DIR/updater.log
    Environment=TASK_FILE_NAME=model.onnx
    Environment=TASK_DIR_PATH=$MINER_TASK_DIR
    Environment=IDENTITY_FILE_PATH=$MINER_CONFIG_DIR/miner_identity.json
    Environment=TASK_OWNER_FILE_PATH=$MINER_CONFIG_DIR/task_owner.json
    Environment=CURRENT_TASK_PATH=$MINER_CONFIG_DIR/current_task.json
    Environment=TAILSCALE_NET=$TAILSCALE_NET
    Environment=FLASH_INFER_PORT=$FLASH_INFER_PORT
    Environment=MINER_TYPE=$MINER_TYPE
    ExecStart=$MINER_BINARY_PATH start-miner --parachain-url \$PARACHAIN_URL --account-seed "\$ACCOUNT_SEED" --miner-type $MINER_TYPE
    Restart=always
    SuccessExitStatus=75
    RestartSec=3

    [Install]
    WantedBy=multi-user.target
EOL

    echo "systemd service for $MINER_FILE_NAME created successfully!"

    echo "Creating systemd service for agent: $AGENT_SERVICE_FILE"

    bash -c "cat > $AGENT_SERVICE_FILE" << EOL
    [Unit]
    Description=Agent that is able to check the health of the miner, provide required info to the cyborg-parachain, and stream usage metrics and logs of the cyborg node.
    After=network.target

    [Service]
    User=root
    Group=root
    SupplementaryGroups=docker
    Environment=LOG_FILE_PATH=$MINER_LOG_DIR/miner.log
    Environment=TASK_OWNER_FILE_PATH=$MINER_CONFIG_DIR/task_owner.json
    Environment=IDENTITY_FILE_PATH=$MINER_CONFIG_DIR/miner_identity.json
    ExecStart=$AGENT_BINARY_PATH run
    Restart=always
    RestartSec=3

    [Install]
    WantedBy=multi-user.target
EOL

    echo "systemd service for $AGENT_FILE_NAME created successfully!"

    echo "Reloading systemd, enabling and starting $MINER_FILE_NAME and $AGENT_FILE_NAME services..."

    systemctl daemon-reexec
    systemctl daemon-reload
    systemctl enable "$MINER_FILE_NAME"
    systemctl enable "$AGENT_FILE_NAME"
    systemctl restart "$MINER_FILE_NAME"
    systemctl restart "$AGENT_FILE_NAME"

    systemctl status "$MINER_FILE_NAME" --no-pager
    systemctl status "$AGENT_FILE_NAME" --no-pager

    echo "Cyborg Miner and Agent are installed and running. Binaries are located at $MINER_BINARY_PATH and $AGENT_BINARY_PATH. Now attempting to open Port $AGENT_HTTP_PORT, $AGENT_WS_PORT and $MINER_INFERENCE_PORT to enable communication with Cyborg Connect and provide an inference endpoint."
}

move_files() {
    echo "Moving the miner to $BIN_DIR..."
    echo "Moving the agent to $BIN_DIR..."
    echo "Moving the setup script to $SCRIPT_DIR..."

    mv "$MINER_BIN" "$MINER_BINARY_PATH"
    mv "$AGENT_BIN" "$AGENT_BINARY_PATH"
    mv "$SETUP_SCRIPT" "$SETUP_SCRIPT_PATH"
}

open_firewall() {
    if command -v ufw &> /dev/null; then
        FIREWALL="ufw"
    elif command -v firewall-cmd &> /dev/null; then
        FIREWALL="firewalld"
    elif command -v iptables &> /dev/null; then
        FIREWALL="iptables"
    else
        echo "Firewall management tool not detected. Please open $AGENT_HTTP_PORT, $AGENT_WS_PORT and $MINER_INFERENCE_PORT manually for the miner to work."
        echo "If in doubt, refer to the documentation of your firewall management tool for instructions."
    fi

    open_ports_ufw() {
        ufw allow $AGENT_WS_PORT
        ufw allow $AGENT_HTTP_PORT
        ufw allow $MINER_INFERENCE_PORT
        echo "Ports opened in UFW."
    }

    # Function to open ports with firewalld
    open_ports_firewalld() {
        firewall-cmd --permanent --add-port=$AGENT_HTTP_PORT/tcp
        firewall-cmd --permanent --add-port=$AGENT_WS_PORT/tcp
        firewall-cmd --permanent --add-port=$MINER_INFERENCE_PORT/tcp
        firewall-cmd --reload
        echo "Ports opened in firewalld."
    }

    # Function to open ports with iptables
    open_ports_iptables() {
        iptables -A INPUT -p tcp --dport $AGENT_HTTP_PORT -j ACCEPT
        iptables -A INPUT -p tcp --dport $AGENT_WS_PORT -j ACCEPT
        iptables -A INPUT -p tcp --dport $MINER_INFERENCE_PORT -j ACCEPT
        # Note: Rules added with iptables are not persistent across reboots unless saved.
        echo "Ports opened in iptables."
    }

    if [[ -n "${FIREWALL:-}" ]]; then
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
    fi
}

install() {
    echo "Initiating miner registration..."

    PARACHAIN_URL="${PARACHAIN_URL:-}"
    ACCOUNT_SEED="${ACCOUNT_SEED:-}"
    MINER_TYPE="${MINER_TYPE:-}"

    if [[ -z "$PARACHAIN_URL" || -z "$ACCOUNT_SEED" || -z "$MINER_TYPE" ]]; then
        echo "ERROR: PARACHAIN_URL and ACCOUNT_SEED must be set in environment."
        exit 1
    fi

    download_and_extract
    prepare_environment
    setup_docker
    move_files
    setup_systemd "$PARACHAIN_URL" "$ACCOUNT_SEED" "$MINER_TYPE"
    open_firewall
    #prepare_triton
}

update() {
    CURRENT_VERSION="$1"

    ################### WARNING this is not safe, we need proper key management ##########################
    echo "Reading current configuration from systemd service file..."
    SERVICE_FILE="/etc/systemd/system/cyborg-miner.service"

    if [[ ! -f "$SERVICE_FILE" ]]; then
        echo "Service file not found: $SERVICE_FILE"
        echo "Cannot extract PARACHAIN_URL or ACCOUNT_SEED."
        exit 1
    fi

    PARACHAIN_URL=$(systemctl show cyborg-miner.service -p Environment | grep -o 'PARACHAIN_URL=[^ ]*' | cut -d= -f2)
    ACCOUNT_SEED=$(systemctl show cyborg-miner.service -p Environment | grep -o 'ACCOUNT_SEED=[^ ]*' | cut -d= -f2)
    MINER_TYPE=$(systemctl show cyborg-miner.service -p Environment | grep -o 'MINER_TYPE=[^ ]*' | cut -d= -f2)

    if [[ -z "$PARACHAIN_URL" || -z "$ACCOUNT_SEED" || -z "$MINER_TYPE" ]]; then
        echo "Failed to extract required variables from $SERVICE_FILE"
        exit 1
    fi

    echo "PARACHAIN_URL: $PARACHAIN_URL"
    echo "ACCOUNT_SEED: $ACCOUNT_SEED"
    echo "MINER_TYPE: $MINER_TYPE"

    ###############################################################################################################

    echo "Current version: $CURRENT_VERSION"
    echo "Fetching latest release tag..."

    TAG=$(curl -s https://api.github.com/repos/${REPO}/releases/latest | grep -Po '"tag_name": "\K.*?(?=")')
    echo "Latest version available: $TAG"

    if [[ "$TAG" == "v$CURRENT_VERSION" || "$TAG" == "$CURRENT_VERSION" ]]; then
        echo "Already up-to-date (version $CURRENT_VERSION)."
        exit 0
    fi

    echo "Updating from $CURRENT_VERSION to $TAG..."
    download_and_extract
    prepare_environment
    setup_docker

    # Avoid race condition
    systemctl stop cyborg-miner.service
    systemctl stop cyborg-agent.service

    move_files
    setup_systemd "$PARACHAIN_URL" "$ACCOUNT_SEED" "$MINER_TYPE"
    open_firewall

    echo "Update complete: $CURRENT_VERSION to $TAG"
}

# ======================================== DISPATCH ==================================================

case "${1:-install}" in
  install)
    install
    ;;
  update)
    CURRENT_VERSION="${2:-unknown}"
    update "$CURRENT_VERSION"
    ;;
  *)
    echo "Usage: $0 {install|update} [current_version]"
    exit 1
    ;;
esac