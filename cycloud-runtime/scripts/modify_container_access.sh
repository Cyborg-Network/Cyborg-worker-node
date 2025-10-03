#!/bin/bash

set -euo pipefail

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
LOG_FILE="${LOG_FILE:-/var/log/modify-container-access.log}"
LOCK_FILE="/var/run/modify-container-access.lock"
CONFIG_FILE="${CONFIG_FILE:-/etc/container-access-control/config.json}"
MAPPING_DIR="/var/lib/container-ssh-mappings"

# Protected ports and rules that cannot be modified
declare -a PROTECTED_PORTS=(22 443 80)  # Admin SSH, HTTPS, HTTP
declare -a PROTECTED_RULE_PREFIXES=("SSH-Admin" "HTTPS-Admin" "HTTP-Admin" "AllowVnetInBound" "AllowAzureLoadBalancerInBound" "DenyAllInBound")

# Logging functions
log() {
    echo "[$(date +'%Y-%m-%d %H:%M:%S')] $*" | tee -a "$LOG_FILE"
}

error() {
    echo "[$(date +'%Y-%m-%d %H:%M:%S')] ERROR: $*" | tee -a "$LOG_FILE" >&2
}

warning() {
    echo "[$(date +'%Y-%m-%d %H:%M:%S')] WARNING: $*" | tee -a "$LOG_FILE"
}

# Check if running as root
if [[ $EUID -ne 0 ]]; then
   error "This script must be run as root"
   exit 1
fi

# Usage
usage() {
    cat <<EOF
Usage: $0 [OPTIONS]

Modify network access for a container by managing NSG rules and iptables.

Required:
    --container-name NAME       Name of the container
    --action ACTION            Action: open, close, list, status
    
For 'open' action:
    --protocol PROTO           Protocol: tcp, udp, or both
    --port PORT                Port number or range (e.g., 8080 or 8080-8090)
    --source-ip CIDR           Source IP/CIDR (default: '*' for all)

For 'close' action:
    --protocol PROTO           Protocol: tcp, udp, or both
    --port PORT                Port number or range

Optional:
    --resource-group NAME      Azure resource group (auto-detected if not provided)
    --nsg-name NAME           Network Security Group name (auto-detected if not provided)
    --priority NUM            NSG rule priority (auto-assigned if not provided)
    --dry-run                 Show what would be done without making changes
    --force                   Skip confirmation prompts

Examples:
    # Open TCP port 8080 for a container
    $0 --container-name user123 --action open --protocol tcp --port 8080
    
    # Open port range with specific source
    $0 --container-name user123 --action open --protocol tcp --port 8000-8100 --source-ip 10.0.0.0/8
    
    # Close a port
    $0 --container-name user123 --action close --protocol tcp --port 8080
    
    # List all open ports for a container
    $0 --container-name user123 --action list
    
    # Check status
    $0 --container-name user123 --action status

EOF
    exit 1
}

# Initialize variables
CONTAINER_NAME=""
ACTION=""
PROTOCOL=""
PORT=""
SOURCE_IP="*"
RESOURCE_GROUP=""
NSG_NAME=""
PRIORITY=""
DRY_RUN=false
FORCE=false

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --container-name)
            CONTAINER_NAME="$2"
            shift 2
            ;;
        --action)
            ACTION="$2"
            shift 2
            ;;
        --protocol)
            PROTOCOL="$2"
            shift 2
            ;;
        --port)
            PORT="$2"
            shift 2
            ;;
        --source-ip)
            SOURCE_IP="$2"
            shift 2
            ;;
        --resource-group)
            RESOURCE_GROUP="$2"
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
        --dry-run)
            DRY_RUN=true
            shift
            ;;
        --force)
            FORCE=true
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

# Validate required parameters
if [[ -z "$CONTAINER_NAME" ]]; then
    error "Container name is required"
    usage
fi

if [[ -z "$ACTION" ]]; then
    error "Action is required"
    usage
fi

case "$ACTION" in
    open|close)
        if [[ -z "$PROTOCOL" || -z "$PORT" ]]; then
            error "Protocol and port are required for $ACTION action"
            usage
        fi
        ;;
    list|status)
        # No additional parameters needed
        ;;
    *)
        error "Invalid action: $ACTION. Must be one of: open, close, list, status"
        usage
        ;;
esac

# Acquire lock
exec 200>"$LOCK_FILE"
if ! flock -n 200; then
    error "Another instance is already running"
    exit 1
fi

# Cleanup handler
cleanup_handler() {
    local exit_code=$?
    flock -u 200
    if [[ $exit_code -ne 0 ]]; then
        error "Script failed with exit code $exit_code"
    fi
    exit $exit_code
}
trap cleanup_handler EXIT

# Validate protocol
validate_protocol() {
    local proto="$1"
    case "${proto,,}" in
        tcp|udp|both)
            return 0
            ;;
        *)
            error "Invalid protocol: $proto. Must be tcp, udp, or both"
            return 1
            ;;
    esac
}

# Validate port
validate_port() {
    local port="$1"
    
    # Check if it's a range
    if [[ "$port" =~ ^([0-9]+)-([0-9]+)$ ]]; then
        local start="${BASH_REMATCH[1]}"
        local end="${BASH_REMATCH[2]}"
        
        if [[ $start -lt 1 || $start -gt 65535 || $end -lt 1 || $end -gt 65535 ]]; then
            error "Port numbers must be between 1 and 65535"
            return 1
        fi
        
        if [[ $start -ge $end ]]; then
            error "Invalid port range: start port must be less than end port"
            return 1
        fi
    elif [[ "$port" =~ ^[0-9]+$ ]]; then
        if [[ $port -lt 1 || $port -gt 65535 ]]; then
            error "Port number must be between 1 and 65535"
            return 1
        fi
    else
        error "Invalid port format: $port"
        return 1
    fi
    
    return 0
}

# Check if port is protected
is_port_protected() {
    local port="$1"
    
    # Extract single port from range for checking
    local check_port="$port"
    if [[ "$port" =~ ^([0-9]+)-([0-9]+)$ ]]; then
        check_port="${BASH_REMATCH[1]}"
    fi
    
    for protected in "${PROTECTED_PORTS[@]}"; do
        if [[ "$check_port" == "$protected" ]]; then
            return 0
        fi
    done
    
    return 1
}

# Check if NSG rule is protected
is_rule_protected() {
    local rule_name="$1"
    
    for prefix in "${PROTECTED_RULE_PREFIXES[@]}"; do
        if [[ "$rule_name" =~ ^${prefix} ]]; then
            return 0
        fi
    done
    
    return 1
}

# Load container mapping
load_container_mapping() {
    local container="$1"
    local mapping_file="${MAPPING_DIR}/${container}.json"
    
    if [[ ! -f "$mapping_file" ]]; then
        error "No mapping found for container $container. Has it been set up?"
        return 1
    fi
    
    cat "$mapping_file"
    return 0
}

# Auto-detect Azure metadata
detect_azure_metadata() {
    log "Detecting Azure metadata..."
    
    local metadata_endpoint="http://169.254.169.254/metadata/instance?api-version=2021-02-01"
    local metadata
    
    if ! metadata=$(curl -s -H "Metadata:true" --noproxy "*" "$metadata_endpoint" 2>/dev/null); then
        error "Failed to retrieve Azure metadata"
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
    
    return 0
}

# Detect NSG
detect_nsg() {
    log "Detecting Network Security Group..."
    
    local nic_id=$(az vm show --resource-group "$RESOURCE_GROUP" --name "$VM_NAME" \
        --query 'networkProfile.networkInterfaces[0].id' -o tsv 2>/dev/null)
    
    if [[ -z "$nic_id" ]]; then
        error "Failed to get network interface ID"
        return 1
    fi
    
    local nsg_id=$(az network nic show --ids "$nic_id" \
        --query 'networkSecurityGroup.id' -o tsv 2>/dev/null)
    
    if [[ -z "$nsg_id" || "$nsg_id" == "null" ]]; then
        local subnet_id=$(az network nic show --ids "$nic_id" \
            --query 'ipConfigurations[0].subnet.id' -o tsv 2>/dev/null)
        
        nsg_id=$(az network vnet subnet show --ids "$subnet_id" \
            --query 'networkSecurityGroup.id' -o tsv 2>/dev/null)
    fi
    
    if [[ -z "$nsg_id" || "$nsg_id" == "null" ]]; then
        error "No Network Security Group found"
        return 1
    fi
    
    NSG_NAME=$(basename "$nsg_id")
    log "Detected NSG: $NSG_NAME"
    return 0
}

# Get container IP
get_container_ip() {
    local container_name="$1"
    local container_ip
    
    container_ip=$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' "$container_name" 2>/dev/null)
    
    if [[ -z "$container_ip" ]]; then
        error "Failed to get IP address for container $container_name. Is it running?"
        return 1
    fi
    
    echo "$container_ip"
    return 0
}

# Find available priority
find_available_priority() {
    log "Finding available NSG rule priority..."
    
    local used_priorities=$(az network nsg rule list \
        --resource-group "$RESOURCE_GROUP" \
        --nsg-name "$NSG_NAME" \
        --query '[].priority' -o tsv | sort -n)
    
    local priority=1100
    while echo "$used_priorities" | grep -q "^${priority}$"; do
        ((priority++))
        if [[ $priority -gt 4096 ]]; then
            error "No available priority found"
            return 1
        fi
    done
    
    echo "$priority"
    return 0
}

# Generate rule name
generate_rule_name() {
    local container="$1"
    local protocol="$2"
    local port="$3"
    
    # Sanitize port for rule name (replace - with _)
    local port_clean="${port//-/_}"
    echo "Container-${container}-${protocol^^}-${port_clean}"
}

# Create NSG rule
create_nsg_rule() {
    local rule_name="$1"
    local priority="$2"
    local protocol="$3"
    local port="$4"
    local source="$5"
    
    # Check if rule is protected
    if is_rule_protected "$rule_name"; then
        error "Cannot create protected rule: $rule_name"
        return 1
    fi
    
    log "Creating NSG rule: $rule_name"
    
    if [[ "$DRY_RUN" == true ]]; then
        log "[DRY RUN] Would create NSG rule: $rule_name (priority: $priority, protocol: $protocol, port: $port, source: $source)"
        return 0
    fi
    
    local protocol_param="$protocol"
    if [[ "${protocol,,}" == "both" ]]; then
        protocol_param="*"
    fi
    
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
            --protocol "$protocol_param" \
            --access Allow \
            --direction Inbound \
            --source-address-prefixes "$source" \
            --destination-address-prefixes '*' \
            --description "Container $CONTAINER_NAME access" \
            -o none
    else
        az network nsg rule create \
            --resource-group "$RESOURCE_GROUP" \
            --nsg-name "$NSG_NAME" \
            --name "$rule_name" \
            --priority "$priority" \
            --destination-port-ranges "$port" \
            --protocol "$protocol_param" \
            --access Allow \
            --direction Inbound \
            --source-address-prefixes "$source" \
            --destination-address-prefixes '*' \
            --description "Container $CONTAINER_NAME access" \
            -o none
    fi
    
    log "NSG rule created/updated successfully"
    return 0
}

# Delete NSG rule
delete_nsg_rule() {
    local rule_name="$1"
    
    # Check if rule is protected
    if is_rule_protected "$rule_name"; then
        error "Cannot delete protected rule: $rule_name"
        return 1
    fi
    
    log "Deleting NSG rule: $rule_name"
    
    if [[ "$DRY_RUN" == true ]]; then
        log "[DRY RUN] Would delete NSG rule: $rule_name"
        return 0
    fi
    
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
        warning "NSG rule $rule_name does not exist"
    fi
    
    return 0
}

# Setup iptables forwarding
setup_iptables_forwarding() {
    local protocol="$1"
    local external_port="$2"
    local container_ip="$3"
    local internal_port="$4"
    
    log "Setting up iptables forwarding: ${protocol}/${external_port} -> ${container_ip}:${internal_port}"
    
    if [[ "$DRY_RUN" == true ]]; then
        log "[DRY RUN] Would setup iptables forwarding"
        return 0
    fi
    
    # Enable IP forwarding if not already enabled
    sysctl -w net.ipv4.ip_forward=1 &>/dev/null
    
    # Handle protocol
    local -a protocols=()
    if [[ "${protocol,,}" == "both" ]]; then
        protocols=(tcp udp)
    else
        protocols=("${protocol,,}")
    fi
    
    for proto in "${protocols[@]}"; do
        # PREROUTING: Forward external traffic to container
        iptables -t nat -C PREROUTING -p "$proto" --dport "$external_port" -j DNAT --to-destination "${container_ip}:${internal_port}" 2>/dev/null || \
        iptables -t nat -A PREROUTING -p "$proto" --dport "$external_port" -j DNAT --to-destination "${container_ip}:${internal_port}"
        
        # POSTROUTING: Masquerade for return traffic
        iptables -t nat -C POSTROUTING -p "$proto" -d "$container_ip" --dport "$internal_port" -j MASQUERADE 2>/dev/null || \
        iptables -t nat -A POSTROUTING -p "$proto" -d "$container_ip" --dport "$internal_port" -j MASQUERADE
        
        # FORWARD: Allow forwarding
        iptables -C FORWARD -p "$proto" -d "$container_ip" --dport "$internal_port" -j ACCEPT 2>/dev/null || \
        iptables -A FORWARD -p "$proto" -d "$container_ip" --dport "$internal_port" -j ACCEPT
    done
    
    # Save iptables rules
    if command -v iptables-save &>/dev/null; then
        iptables-save > /etc/iptables/rules.v4 2>/dev/null || true
    fi
    
    log "iptables forwarding configured successfully"
    return 0
}

# Remove iptables forwarding
remove_iptables_forwarding() {
    local protocol="$1"
    local external_port="$2"
    local container_ip="$3"
    local internal_port="$4"
    
    log "Removing iptables forwarding: ${protocol}/${external_port}"
    
    if [[ "$DRY_RUN" == true ]]; then
        log "[DRY RUN] Would remove iptables forwarding"
        return 0
    fi
    
    # Handle protocol
    local -a protocols=()
    if [[ "${protocol,,}" == "both" ]]; then
        protocols=(tcp udp)
    else
        protocols=("${protocol,,}")
    fi
    
    for proto in "${protocols[@]}"; do
        iptables -t nat -D PREROUTING -p "$proto" --dport "$external_port" -j DNAT --to-destination "${container_ip}:${internal_port}" 2>/dev/null || true
        iptables -t nat -D POSTROUTING -p "$proto" -d "$container_ip" --dport "$internal_port" -j MASQUERADE 2>/dev/null || true
        iptables -D FORWARD -p "$proto" -d "$container_ip" --dport "$internal_port" -j ACCEPT 2>/dev/null || true
    done
    
    # Save iptables rules
    if command -v iptables-save &>/dev/null; then
        iptables-save > /etc/iptables/rules.v4 2>/dev/null || true
    fi
    
    log "iptables forwarding removed successfully"
    return 0
}

# Store port mapping
store_port_mapping() {
    local container="$1"
    local protocol="$2"
    local port="$3"
    local rule_name="$4"
    local priority="$5"
    
    local mappings_file="${MAPPING_DIR}/${container}_ports.json"
    
    # Initialize file if it doesn't exist
    if [[ ! -f "$mappings_file" ]]; then
        echo '{"ports": []}' > "$mappings_file"
    fi
    
    # Add new mapping
    local temp_file=$(mktemp)
    jq --arg proto "$protocol" --arg port "$port" --arg rule "$rule_name" --arg prio "$priority" \
        '.ports += [{
            "protocol": $proto,
            "port": $port,
            "rule_name": $rule,
            "priority": $prio,
            "created_at": now | strftime("%Y-%m-%dT%H:%M:%S%Z")
        }] | .ports |= unique_by(.protocol + .port)' \
        "$mappings_file" > "$temp_file"
    
    mv "$temp_file" "$mappings_file"
    log "Port mapping stored"
}

# Remove port mapping
remove_port_mapping() {
    local container="$1"
    local protocol="$2"
    local port="$3"
    
    local mappings_file="${MAPPING_DIR}/${container}_ports.json"
    
    if [[ ! -f "$mappings_file" ]]; then
        return 0
    fi
    
    local temp_file=$(mktemp)
    jq --arg proto "$protocol" --arg port "$port" \
        '.ports |= map(select(.protocol != $proto or .port != $port))' \
        "$mappings_file" > "$temp_file"
    
    mv "$temp_file" "$mappings_file"
    log "Port mapping removed"
}

# Get port mappings
get_port_mappings() {
    local container="$1"
    local mappings_file="${MAPPING_DIR}/${container}_ports.json"
    
    if [[ ! -f "$mappings_file" ]]; then
        echo '{"ports": []}'
        return 0
    fi
    
    cat "$mappings_file"
}

# Action: Open port
action_open() {
    log "Opening port for container: $CONTAINER_NAME"
    
    # Validate inputs
    if ! validate_protocol "$PROTOCOL"; then
        return 1
    fi
    
    if ! validate_port "$PORT"; then
        return 1
    fi
    
    # Check if port is protected
    if is_port_protected "$PORT"; then
        error "Cannot open protected port: $PORT"
        return 1
    fi
    
    # Load container mapping
    local mapping
    if ! mapping=$(load_container_mapping "$CONTAINER_NAME"); then
        return 1
    fi
    
    # Get stored Azure info
    local stored_rg=$(echo "$mapping" | jq -r '.resource_group')
    local stored_nsg=$(echo "$mapping" | jq -r '.nsg_name')
    RESOURCE_GROUP="${RESOURCE_GROUP:-$stored_rg}"
    NSG_NAME="${NSG_NAME:-$stored_nsg}"
    
    # Get container IP
    local container_ip
    if ! container_ip=$(get_container_ip "$CONTAINER_NAME"); then
        return 1
    fi
    
    # Generate rule name
    local rule_name=$(generate_rule_name "$CONTAINER_NAME" "$PROTOCOL" "$PORT")
    
    # Get or find priority
    if [[ -z "$PRIORITY" ]]; then
        if ! PRIORITY=$(find_available_priority); then
            return 1
        fi
    fi
    
    # Confirm action
    if [[ "$FORCE" == false && "$DRY_RUN" == false ]]; then
        echo ""
        echo "About to open port with the following configuration:"
        echo "  Container: $CONTAINER_NAME"
        echo "  Protocol: $PROTOCOL"
        echo "  Port: $PORT"
        echo "  Source: $SOURCE_IP"
        echo "  Container IP: $container_ip"
        echo "  NSG Rule: $rule_name"
        echo ""
        read -p "Continue? (yes/no): " confirm
        if [[ "$confirm" != "yes" ]]; then
            log "Operation cancelled by user"
            return 1
        fi
    fi
    
    # Create NSG rule
    if ! create_nsg_rule "$rule_name" "$PRIORITY" "$PROTOCOL" "$PORT" "$SOURCE_IP"; then
        return 1
    fi
    
    # Setup iptables forwarding (port to port mapping)
    if ! setup_iptables_forwarding "$PROTOCOL" "$PORT" "$container_ip" "$PORT"; then
        error "Failed to setup iptables, rolling back NSG rule"
        delete_nsg_rule "$rule_name"
        return 1
    fi
    
    # Store mapping
    if [[ "$DRY_RUN" == false ]]; then
        store_port_mapping "$CONTAINER_NAME" "$PROTOCOL" "$PORT" "$rule_name" "$PRIORITY"
    fi
    
    log "Port opened successfully"
    return 0
}

# Action: Close port
action_close() {
    log "Closing port for container: $CONTAINER_NAME"
    
    # Validate inputs
    if ! validate_protocol "$PROTOCOL"; then
        return 1
    fi
    
    if ! validate_port "$PORT"; then
        return 1
    fi
    
    # Check if port is protected
    if is_port_protected "$PORT"; then
        error "Cannot close protected port: $PORT"
        return 1
    fi
    
    # Load container mapping
    local mapping
    if ! mapping=$(load_container_mapping "$CONTAINER_NAME"); then
        return 1
    fi
    
    # Get stored Azure info
    local stored_rg=$(echo "$mapping" | jq -r '.resource_group')
    local stored_nsg=$(echo "$mapping" | jq -r '.nsg_name')
    RESOURCE_GROUP="${RESOURCE_GROUP:-$stored_rg}"
    NSG_NAME="${NSG_NAME:-$stored_nsg}"
    
    # Get container IP
    local container_ip
    if ! container_ip=$(get_container_ip "$CONTAINER_NAME"); then
        return 1
    fi
    
    # Generate rule name
    local rule_name=$(generate_rule_name "$CONTAINER_NAME" "$PROTOCOL" "$PORT")
    
    # Confirm action
    if [[ "$FORCE" == false && "$DRY_RUN" == false ]]; then
        echo ""
        echo "About to close port with the following configuration:"
        echo "  Container: $CONTAINER_NAME"
        echo "  Protocol: $PROTOCOL"
        echo "  Port: $PORT"
        echo "  NSG Rule: $rule_name"
        echo ""
        read -p "Continue? (yes/no): " confirm
        if [[ "$confirm" != "yes" ]]; then
            log "Operation cancelled by user"
            return 1
        fi
    fi
    
    # Remove iptables forwarding
    remove_iptables_forwarding "$PROTOCOL" "$PORT" "$container_ip" "$PORT"
    
    # Delete NSG rule
    if ! delete_nsg_rule "$rule_name"; then
        warning "Failed to delete NSG rule, but iptables rules were removed"
    fi
    
    # Remove mapping
    if [[ "$DRY_RUN" == false ]]; then
        remove_port_mapping "$CONTAINER_NAME" "$PROTOCOL" "$PORT"
    fi
    
    log "Port closed successfully"
    return 0
}

# Action: List ports
action_list() {
    log "Listing open ports for container: $CONTAINER_NAME"
    
    # Load container mapping
    local mapping
    if ! mapping=$(load_container_mapping "$CONTAINER_NAME"); then
        return 1
    fi
    
    # Get port mappings
    local port_mappings=$(get_port_mappings "$CONTAINER_NAME")
    
    echo ""
    echo "Container: $CONTAINER_NAME"
    echo "Status: $(docker inspect -f '{{.State.Status}}' "$CONTAINER_NAME" 2>/dev/null || echo 'UNKNOWN')"
    echo "IP Address: $(get_container_ip "$CONTAINER_NAME" 2>/dev/null || echo 'N/A')"
    echo ""
    echo "Open Ports:"
    echo "==========="
    
    local port_count=$(echo "$port_mappings" | jq '.ports | length')
    
    if [[ $port_count -eq 0 ]]; then
        echo "  No additional ports open (SSH port may be configured separately)"
    else
        echo "$port_mappings" | jq -r '.ports[] | "  \(.protocol | ascii_upcase)/\(.port) - Rule: \(.rule_name) (Priority: \(.priority))"'
    fi
    
    echo ""
    return 0
}

# Action: Status
action_status() {
    log "Checking status for container: $CONTAINER_NAME"
    
    # Load container mapping
    local mapping
    if ! mapping=$(load_container_mapping "$CONTAINER_NAME"); then
        return 1
    fi
    
    # Get stored info
    local ssh_port=$(echo "$mapping" | jq -r '.ssh_port')
    local stored_rg=$(echo "$mapping" | jq -r '.resource_group')
    local stored_nsg=$(echo "$mapping" | jq -r '.nsg_name')
    RESOURCE_GROUP="${RESOURCE_GROUP:-$stored_rg}"
    NSG_NAME="${NSG_NAME:-$stored_nsg}"
    
    # Get container status
    local container_status=$(docker inspect -f '{{.State.Status}}' "$CONTAINER_NAME" 2>/dev/null || echo 'NOT_FOUND')
    local container_ip=$(get_container_ip "$CONTAINER_NAME" 2>/dev/null || echo 'N/A')
    
    # Get port mappings
    local port_mappings=$(get_port_mappings "$CONTAINER_NAME")
    local port_count=$(echo "$port_mappings" | jq '.ports | length')
    
    # Get NSG rules for this container
    local nsg_rules=$(az network nsg rule list \
        --resource-group "$RESOURCE_GROUP" \
        --nsg-name "$NSG_NAME" \
        --query "[?contains(name, 'Container-$CONTAINER_NAME')]" \
        -o json 2>/dev/null || echo '[]')
    local nsg_rule_count=$(echo "$nsg_rules" | jq 'length')
    
    echo ""
    echo "Container Access Status"
    echo "======================="
    echo "Container Name: $CONTAINER_NAME"
    echo "Container Status: $container_status"
    echo "Container IP: $container_ip"
    echo "SSH Port: $ssh_port"
    echo ""
    echo "Azure Configuration:"
    echo "  Resource Group: $RESOURCE_GROUP"
    echo "  NSG Name: $NSG_NAME"
    echo ""
    echo "Network Access:"
    echo "  Additional Ports Open: $port_count"
    echo "  NSG Rules: $nsg_rule_count"
    echo ""
    
    if [[ $port_count -gt 0 ]]; then
        echo "Open Ports:"
        echo "$port_mappings" | jq -r '.ports[] | "  \(.protocol | ascii_upcase)/\(.port)"'
        echo ""
    fi
    
    # Check for mismatches
    local warnings=0
    if [[ $port_count -ne $nsg_rule_count ]]; then
        warning "Port count mismatch: $port_count ports tracked, $nsg_rule_count NSG rules found"
        ((warnings++))
    fi
    
    if [[ "$container_status" != "running" ]]; then
        warning "Container is not running: $container_status"
        ((warnings++))
    fi
    
    if [[ $warnings -gt 0 ]]; then
        echo "⚠️  $warnings warning(s) detected"
    else
        echo "✓ All checks passed"
    fi
    
    echo ""
    return 0
}

main() {
    log "=== Modify Container Access Script Starting ==="
    log "Container: $CONTAINER_NAME, Action: $ACTION"
    
    # Check dependencies
    for cmd in az docker jq iptables curl; do
        if ! command -v "$cmd" &>/dev/null; then
            error "Required command not found: $cmd"
            exit 1
        fi
    done
    
    # Check Azure CLI login
    if ! az account show &>/dev/null; then
        error "Not logged in to Azure CLI. Use managed identity or 'az login'"
        exit 1
    fi
    
    # Check if mapping directory exists
    if [[ ! -d "$MAPPING_DIR" ]]; then
        error "Mapping directory not found: $MAPPING_DIR"
        error "Has the container been set up using the setup script?"
        exit 1
    fi
    
    case "$ACTION" in
        open)
            action_open
            ;;
        close)
            action_close
            ;;
        list)
            action_list
            ;;
        status)
            action_status
            ;;
        *)
            error "Invalid action: $ACTION"
            exit 1
            ;;
    esac
    
    local exit_code=$?
    
    if [[ $exit_code -eq 0 ]]; then
        log "=== Script completed successfully ==="
    else
        log "=== Script completed with errors ==="
    fi
    
    return $exit_code
}

main