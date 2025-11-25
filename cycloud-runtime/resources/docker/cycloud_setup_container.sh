#!/bin/bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
LOG_FILE="${LOG_FILE:-/var/log/container-ssh-setup.log}"
LOCK_FILE="/var/run/container-ssh-setup.lock"

log() {
    echo "[$(date +'%Y-%m-%d %H:%M:%S')] $*" | tee -a "$LOG_FILE"
}

error() {
    echo "[$(date +'%Y-%m-%d %H:%M:%S')] ERROR: $*" | tee -a "$LOG_FILE" >&2
}

# Check if running as root
if [[ $EUID -ne 0 ]]; then
   error "This script must be run as root"
   exit 1
fi

usage() {
    cat <<EOF
Usage: $0 [OPTIONS]

Required:
    --container-name NAME       Name of the container
    --ssh-public-key KEY        User's SSH public key
    --ssh-port PORT             External SSH port to expose

Optional:
    --resource-group NAME       Azure resource group (auto-detected if not provided)
    --vm-name NAME              Azure VM name (auto-detected if not provided)
    --nsg-name NAME             Network Security Group name (auto-detected if not provided)
    --cleanup                   Remove existing setup for this container
    --priority NUM              NSG rule priority (default: auto-assigned)
    --platform PLATFORM         Platform (default: generic; options: azure, generic)

Examples:
    $0 --container-name user123 --ssh-public-key "ssh-rsa AAA..." --ssh-port 2222
    $0 --container-name user123 --cleanup
EOF
    exit 1
}

CONTAINER_NAME=""
SSH_PUBLIC_KEY=""
SSH_PORT=""
RESOURCE_GROUP=""
VM_NAME=""
NSG_NAME=""
CLEANUP=false
PRIORITY=""
PLATFORM="generic"

while [[ $# -gt 0 ]]; do
    case $1 in
        --platform)
            PLATFORM="$2"
            shift 2
            ;;
        --container-name)
            CONTAINER_NAME="$2"
            shift 2
            ;;
        --ssh-public-key)
            SSH_PUBLIC_KEY="$2"
            shift 2
            ;;
        --ssh-port)
            SSH_PORT="$2"
            shift 2
            ;;
        --resource-group)
            RESOURCE_GROUP="$2"
            shift 2
            ;;
        --vm-name)
            VM_NAME="$2"
            shift 2
            ;;
        --nsg-name)
            NSG_NAME="$2"
            shift 2
            ;;
        --priority)
            PRIORITY="$2"
            shift 2
            ;;
        --cleanup)
            CLEANUP=true
            shift
            ;;
        -h|--help)
            usage
            ;;
        *)
            error "Unknown option: $1"
            usage
            ;;
    esac
done

if [[ -z "$CONTAINER_NAME" ]]; then
    error "Container name is required"
    usage
fi

if [[ "$CLEANUP" == false ]] && [[ -z "$SSH_PUBLIC_KEY" || -z "$SSH_PORT" ]]; then
    error "SSH public key and port are required for setup"
    usage
fi

# Acquire lock to prevent concurrent executions
exec 200>"$LOCK_FILE"
if ! flock -n 200; then
    error "Another instance is already running"
    exit 1
fi

cleanup_handler() {
    local exit_code=$?
    flock -u 200
    if [[ $exit_code -ne 0 ]]; then
        error "Script failed with exit code $exit_code"
    fi
    exit $exit_code
}
trap cleanup_handler EXIT

detect_azure_metadata() {
    log "Detecting Azure metadata..."
    
    local metadata_endpoint="http://169.254.169.254/metadata/instance?api-version=2021-02-01"
    local metadata
    
    if ! metadata=$(curl -s -H "Metadata:true" --noproxy "*" "$metadata_endpoint" 2>/dev/null); then
        error "Failed to retrieve Azure metadata. Is this running on an Azure VM?"
        return 1
    fi
    
    if [[ -z "$RESOURCE_GROUP" ]]; then
        RESOURCE_GROUP=$(echo "$metadata" | jq -r '.compute.resourceGroupName')
        log "Detected resource group: $RESOURCE_GROUP"
    fi
    
    if [[ -z "$VM_NAME" ]]; then
        VM_NAME=$(echo "$metadata" | jq -r '.compute.name')
        log "Detected VM name: $VM_NAME"
    fi
    
    # Get network interface info
    local nic_id=$(echo "$metadata" | jq -r '.network.interface[0].ipv4.subnet[0].address')
    log "VM private IP: $nic_id"
    
    return 0
}

detect_nsg() {
    log "Detecting Network Security Group..."
    
    # Get the nic id
    local nic_id=$(az vm show --resource-group "$RESOURCE_GROUP" --name "$VM_NAME" \
        --query 'networkProfile.networkInterfaces[0].id' -o tsv 2>/dev/null)
    
    if [[ -z "$nic_id" ]]; then
        error "Failed to get network interface ID"
        return 1
    fi
    
    # Get nsg from nic
    local nsg_id=$(az network nic show --ids "$nic_id" \
        --query 'networkSecurityGroup.id' -o tsv 2>/dev/null)
    
    if [[ -z "$nsg_id" || "$nsg_id" == "null" ]]; then
        # Try to get NSG from subnet
        local subnet_id=$(az network nic show --ids "$nic_id" \
            --query 'ipConfigurations[0].subnet.id' -o tsv 2>/dev/null)
        
        nsg_id=$(az network vnet subnet show --ids "$subnet_id" \
            --query 'networkSecurityGroup.id' -o tsv 2>/dev/null)
    fi
    
    if [[ -z "$nsg_id" || "$nsg_id" == "null" ]]; then
        error "No Network Security Group found attached to VM or subnet"
        return 1
    fi
    
    NSG_NAME=$(basename "$nsg_id")
    log "Detected NSG: $NSG_NAME"
    return 0
}

find_available_priority() {
    log "Finding available NSG rule priority..."
    
    local used_priorities=$(az network nsg rule list \
        --resource-group "$RESOURCE_GROUP" \
        --nsg-name "$NSG_NAME" \
        --query '[].priority' -o tsv | sort -n)
    
    # Start from 1000 and find first available
    local priority=1000
    while echo "$used_priorities" | grep -q "^${priority}$"; do
        ((priority++))
        if [[ $priority -gt 4096 ]]; then
            error "No available priority found (all priorities 1000-4096 are used)"
            return 1
        fi
    done
    
    echo "$priority"
    return 0
}

create_nsg_rule() {
    local port=$1
    local priority=$2
    local rule_name="SSH-Container-${CONTAINER_NAME}-${port}"
    
    log "Creating NSG rule: $rule_name (priority: $priority, port: $port)"
    
    if az network nsg rule show \
        --resource-group "$RESOURCE_GROUP" \
        --nsg-name "$NSG_NAME" \
        --name "$rule_name" &>/dev/null; then
        log "NSG rule $rule_name already exists, updating..."
        az network nsg rule update \
            --resource-group "$RESOURCE_GROUP" \
            --nsg-name "$NSG_NAME" \
            --name "$rule_name" \
            --priority "$priority" \
            --destination-port-ranges "$port" \
            --protocol Tcp \
            --access Allow \
            --direction Inbound \
            --source-address-prefixes '*' \
            --destination-address-prefixes '*' \
            --description "SSH access to container $CONTAINER_NAME" \
            -o none
    else
        az network nsg rule create \
            --resource-group "$RESOURCE_GROUP" \
            --nsg-name "$NSG_NAME" \
            --name "$rule_name" \
            --priority "$priority" \
            --destination-port-ranges "$port" \
            --protocol Tcp \
            --access Allow \
            --direction Inbound \
            --source-address-prefixes '*' \
            --destination-address-prefixes '*' \
            --description "SSH access to container $CONTAINER_NAME" \
            -o none
    fi
    
    log "NSG rule created successfully"
    return 0
}

delete_nsg_rule() {
    local port=$1
    local rule_name="SSH-Container-${CONTAINER_NAME}-${port}"
    
    log "Deleting NSG rule: $rule_name"
    
    if az network nsg rule show \
        --resource-group "$RESOURCE_GROUP" \
        --nsg-name "$NSG_NAME" \
        --name "$rule_name" &>/dev/null; then
        az network nsg rule delete \
            --resource-group "$RESOURCE_GROUP" \
            --nsg-name "$NSG_NAME" \
            --name "$rule_name" \
            -o none
        log "NSG rule deleted successfully"
    else
        log "NSG rule $rule_name does not exist, skipping"
    fi
    
    return 0
}

setup_iptables() {
    local external_port=$1
    local container_ip=$2
    local container_port=22
    
    log "Setting up iptables port forwarding: ${external_port} -> ${container_ip}:${container_port}"
    
    # Enable IP forwarding
    sysctl -w net.ipv4.ip_forward=1 &>/dev/null
    echo "net.ipv4.ip_forward=1" > /etc/sysctl.d/99-container-forwarding.conf
    
    # PREROUTING: Forward external traffic to container
    iptables -t nat -C PREROUTING -p tcp --dport "$external_port" -j DNAT --to-destination "${container_ip}:${container_port}" 2>/dev/null || \
    iptables -t nat -A PREROUTING -p tcp --dport "$external_port" -j DNAT --to-destination "${container_ip}:${container_port}"
    
    # POSTROUTING: Masquerade for return traffic
    iptables -t nat -C POSTROUTING -p tcp -d "$container_ip" --dport "$container_port" -j MASQUERADE 2>/dev/null || \
    iptables -t nat -A POSTROUTING -p tcp -d "$container_ip" --dport "$container_port" -j MASQUERADE
    
    # FORWARD: Allow forwarding
    iptables -C FORWARD -p tcp -d "$container_ip" --dport "$container_port" -j ACCEPT 2>/dev/null || \
    iptables -A FORWARD -p tcp -d "$container_ip" --dport "$container_port" -j ACCEPT
    
    # Save iptables rules
    if command -v iptables-save &>/dev/null; then
        iptables-save > /etc/iptables/rules.v4 2>/dev/null || true
    fi
    
    log "iptables rules configured successfully"
    return 0
}

remove_iptables() {
    local external_port=$1
    local container_ip=$2
    local container_port=22
    
    log "Removing iptables port forwarding for port ${external_port}"
    
    iptables -t nat -D PREROUTING -p tcp --dport "$external_port" -j DNAT --to-destination "${container_ip}:${container_port}" 2>/dev/null || true
    iptables -t nat -D POSTROUTING -p tcp -d "$container_ip" --dport "$container_port" -j MASQUERADE 2>/dev/null || true
    iptables -D FORWARD -p tcp -d "$container_ip" --dport "$container_port" -j ACCEPT 2>/dev/null || true
    
    # Save iptables rules
    if command -v iptables-save &>/dev/null; then
        iptables-save > /etc/iptables/rules.v4 2>/dev/null || true
    fi
    
    log "iptables rules removed successfully"
    return 0
}

get_container_ip() {
    local container_name=$1
    local container_ip
    
    container_ip=$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' "$container_name" 2>/dev/null)
    
    if [[ -z "$container_ip" ]]; then
        error "Failed to get IP address for container $container_name"
        return 1
    fi
    
    echo "$container_ip"
    return 0
}

launch_container() {
    log "Launching container using Docker Compose..."
    
    export USER_ID=${USER_ID:-1000}
    export MEM_LIMIT=${MEM_LIMIT:-512m}
    export MEM_SWAP_LIMIT=${MEM_SWAP_LIMIT:-1g}
    export CPU_LIMIT=${CPU_LIMIT:-1}
    export PIDS_LIMIT=${PIDS_LIMIT:-1024}
    export CREATED_DATE=$(date -Iseconds)

    if ! docker compose -f "$SCRIPT_DIR/docker-compose.yml" up -d; then
        error "Failed to launch container"
        return 1
    fi

    log "Container launched successfully"
    return 0
}

setup_container_ssh() {
    local container_name=$1
    local ssh_key=$2
    
    log "Setting up SSH in container $container_name"
    
    local retries=0
    while ! docker inspect -f '{{.State.Running}}' "$container_name" 2>/dev/null | grep -q true; do
        if [[ $retries -ge 30 ]]; then
            error "Container $container_name is not running after 30 seconds"
            return 1
        fi
        log "Waiting for container to be running..."
        sleep 1
        ((retries++))
    done
    
    docker exec "$container_name" bash -c "
        # Create SSH directory
        mkdir -p /root/.ssh
        chmod 700 /root/.ssh
        
        # Add authorized key
        echo '$ssh_key' > /root/.ssh/authorized_keys
        chmod 600 /root/.ssh/authorized_keys
    " 2>&1 | tee -a "$LOG_FILE"
    
    if [[ ${PIPESTATUS[0]} -ne 0 ]]; then
        error "Failed to setup SSH in container"
        return 1
    fi
    
    log "SSH configured successfully in container"
    return 0
}

store_mapping() {
    local container=$1
    local port=$2
    local ip=$3
    
    local mapping_file="/var/lib/container-ssh-mappings/${container}.json"
    mkdir -p "$(dirname "$mapping_file")"
    
    cat > "$mapping_file" <<EOF
{
    "container_name": "$container",
    "ssh_port": $port,
    "container_ip": "$ip",
    "created_at": "$(date -Iseconds)",
    "resource_group": "$RESOURCE_GROUP",
    "nsg_name": "$NSG_NAME"
}
EOF
    
    log "Mapping information stored at $mapping_file"
}

load_mapping() {
    local container=$1
    local mapping_file="/var/lib/container-ssh-mappings/${container}.json"
    
    if [[ ! -f "$mapping_file" ]]; then
        error "No mapping found for container $container"
        return 1
    fi
    
    cat "$mapping_file"
    return 0
}

cleanup_setup() {
    log "Cleaning up setup for container: $CONTAINER_NAME"
    
    local mapping
    if ! mapping=$(load_mapping "$CONTAINER_NAME"); then
        log "No existing mapping found, nothing to cleanup"
        return 0
    fi
    
    local ssh_port=$(echo "$mapping" | jq -r '.ssh_port')
    local container_ip=$(echo "$mapping" | jq -r '.container_ip')
    local stored_rg=$(echo "$mapping" | jq -r '.resource_group')
    local stored_nsg=$(echo "$mapping" | jq -r '.nsg_name')
    
    # Use stored values if current ones are empty
    RESOURCE_GROUP="${RESOURCE_GROUP:-$stored_rg}"
    NSG_NAME="${NSG_NAME:-$stored_nsg}"
    
    # Remove iptables rules
    remove_iptables "$ssh_port" "$container_ip"
    
    # Remove nsg rule
    delete_nsg_rule "$ssh_port"
    
    # Remove mapping file
    rm -f "/var/lib/container-ssh-mappings/${CONTAINER_NAME}.json"
    
    log "Cleanup completed successfully"
    return 0
}

main_setup() {
    log "Starting container SSH setup for: $CONTAINER_NAME"
    
    if [ "$PLATFORM" == "azure" ]; then
        if ! detect_azure_metadata; then
            error "Failed to detect Azure metadata"
            return 1
        fi
    
        if [[ -z "$NSG_NAME" ]]; then
            if ! detect_nsg; then
                error "Failed to detect NSG"
                return 1
            fi
        fi
        
        if [[ -z "$PRIORITY" ]]; then
            if ! PRIORITY=$(find_available_priority); then
                error "Failed to find available priority"
                return 1
            fi
        fi
        
        if ! create_nsg_rule "$SSH_PORT" "$PRIORITY"; then
            error "Failed to create NSG rule"
            return 1
        fi
    fi
    
    if ! setup_container_ssh "$CONTAINER_NAME" "$SSH_PUBLIC_KEY"; then
        error "Failed to setup SSH in container"
        delete_nsg_rule "$SSH_PORT"
        return 1
    fi
    
    local container_ip
    if ! container_ip=$(get_container_ip "$CONTAINER_NAME"); then
        error "Failed to get container IP"
        delete_nsg_rule "$SSH_PORT"
        return 1
    fi
    
    if ! setup_iptables "$SSH_PORT" "$container_ip"; then
        error "Failed to setup iptables"
        delete_nsg_rule "$SSH_PORT"
        return 1
    fi
    
    store_mapping "$CONTAINER_NAME" "$SSH_PORT" "$container_ip"
    
    log "Container SSH setup completed successfully"
    log "SSH access: ssh root@<VM_PUBLIC_IP> -p $SSH_PORT"
    
    return 0
}

main() {
    log "=== Container SSH Setup Script Starting ==="
    log "Running for platform: $PLATFORM"
    log "Container: $CONTAINER_NAME"
    
    for cmd in docker jq ufw curl; do
        if ! command -v "$cmd" &>/dev/null; then
            error "Required command not found: $cmd"
            exit 1
        fi
    done

    if ! docker compose version &>/dev/null; then
        error "Docker Compose plugin not found"
        exit 1
    fi
    
    # Check azure cli login if platform is azure
    if [ "$PLATFORM" == "azure" ] && ! az account show &>/dev/null; then
        error "Not logged in to Azure CLI. Use managed identity or 'az login'"
        exit 1
    fi
    
    if [[ "$CLEANUP" == true ]]; then
        cleanup_setup
    else
        main_setup
    fi
    
    log "=== Script completed successfully ==="
    return 0
}

main