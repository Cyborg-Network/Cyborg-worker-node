#!/bin/bash

set -euo pipefail

# Configuration
API_SOCKET="/var/run/container-access-api.sock"
LOG_FILE="/var/log/container-access-api.log"
AUDIT_LOG="/var/log/container-access-audit.log"
RATE_LIMIT_FILE="/var/lib/container-access-api/rate-limits"
MAX_REQUESTS_PER_MINUTE=10
SCRIPTS_DIR="/opt/container-management"

# Logging
log() {
    echo "[$(date +'%Y-%m-%d %H:%M:%S')] $*" | tee -a "$LOG_FILE"
}

error() {
    echo "[$(date +'%Y-%m-%d %H:%M:%S')] ERROR: $*" | tee -a "$LOG_FILE" >&2
}

audit_log() {
    local container="$1"
    local action="$2"
    local user="$3"
    local result="$4"
    local details="${5:-}"
    
    echo "[$(date +'%Y-%m-%d %H:%M:%S')] AUDIT: container=$container action=$action user=$user result=$result details=\"$details\"" >> "$AUDIT_LOG"
}

# Check if running as root
if [[ $EUID -ne 0 ]]; then
   error "This API must be run as root"
   exit 1
fi

# Get container name from caller
get_caller_container() {
    local pid="$1"
    
    # Try to find which container this process belongs to
    local container_id=$(cat /proc/$pid/cgroup 2>/dev/null | grep -oP 'docker/\K[a-f0-9]{64}' | head -n1)
    
    if [[ -z "$container_id" ]]; then
        return 1
    fi
    
    # Get container name from ID
    docker inspect --format='{{.Name}}' "$container_id" 2>/dev/null | sed 's/^\///'
}

# Rate limiting
check_rate_limit() {
    local container="$1"
    local now=$(date +%s)
    local limit_file="${RATE_LIMIT_FILE}/${container}"
    
    mkdir -p "$(dirname "$limit_file")"
    
    # Clean old entries (older than 1 minute)
    local cutoff=$((now - 60))
    
    if [[ -f "$limit_file" ]]; then
        local count=0
        while IFS= read -r timestamp; do
            if [[ $timestamp -gt $cutoff ]]; then
                ((count++))
            fi
        done < "$limit_file"
        
        if [[ $count -ge $MAX_REQUESTS_PER_MINUTE ]]; then
            error "Rate limit exceeded for container $container"
            return 1
        fi
        
        # Clean old entries
        grep "^[0-9]*$" "$limit_file" 2>/dev/null | \
            awk -v cutoff="$cutoff" '$1 > cutoff' > "${limit_file}.tmp" || true
        mv "${limit_file}.tmp" "$limit_file" 2>/dev/null || true
    fi
    
    # Add current request
    echo "$now" >> "$limit_file"
    return 0
}

# Validate container ownership
validate_container_ownership() {
    local caller_container="$1"
    local target_container="$2"
    
    if [[ "$caller_container" != "$target_container" ]]; then
        error "Container $caller_container attempted to access $target_container"
        audit_log "$target_container" "UNAUTHORIZED_ACCESS" "$caller_container" "DENIED" "Cross-container access attempt"
        return 1
    fi
    
    return 0
}

# Sanitize input
sanitize_input() {
    local input="$1"
    # Remove any shell metacharacters
    echo "$input" | tr -cd '[:alnum:]._-'
}

# Validate port
validate_port() {
    local port="$1"
    
    if [[ "$port" =~ ^([0-9]+)-([0-9]+)$ ]]; then
        local start="${BASH_REMATCH[1]}"
        local end="${BASH_REMATCH[2]}"
        
        if [[ $start -lt 1024 || $start -gt 65535 || $end -lt 1024 || $end -gt 65535 ]]; then
            return 1
        fi
        
        if [[ $start -ge $end ]]; then
            return 1
        fi
    elif [[ "$port" =~ ^[0-9]+$ ]]; then
        if [[ $port -lt 1024 || $port -gt 65535 ]]; then
            return 1
        fi
    else
        return 1
    fi
    
    return 0
}

# Validate protocol
validate_protocol() {
    local protocol="$1"
    case "${protocol,,}" in
        tcp|udp|both)
            return 0
            ;;
        *)
            return 1
            ;;
    esac
}

# Validate source IP
validate_source_ip() {
    local source="$1"
    
    # Allow wildcard
    if [[ "$source" == "*" ]]; then
        return 0
    fi
    
    # Validate CIDR notation
    if [[ "$source" =~ ^([0-9]{1,3}\.){3}[0-9]{1,3}(/[0-9]{1,2})?$ ]]; then
        return 0
    fi
    
    return 1
}

# Execute action with validation
execute_action() {
    local caller_container="$1"
    local action="$2"
    shift 2
    local args=("$@")
    
    case "$action" in
        open)
            execute_open "$caller_container" "${args[@]}"
            ;;
        close)
            execute_close "$caller_container" "${args[@]}"
            ;;
        list)
            execute_list "$caller_container"
            ;;
        status)
            execute_status "$caller_container"
            ;;
        *)
            echo "{\"success\": false, \"error\": \"Invalid action: $action\"}"
            return 1
            ;;
    esac
}

# Execute open action
execute_open() {
    local caller_container="$1"
    local protocol="${2:-}"
    local port="${3:-}"
    local source="${4:-*}"
    
    # Validate inputs
    if [[ -z "$protocol" || -z "$port" ]]; then
        echo "{\"success\": false, \"error\": \"Protocol and port are required\"}"
        audit_log "$caller_container" "open" "$caller_container" "FAILED" "Missing parameters"
        return 1
    fi
    
    protocol=$(sanitize_input "$protocol")
    port=$(sanitize_input "$port")
    source=$(sanitize_input "$source")
    
    if ! validate_protocol "$protocol"; then
        echo "{\"success\": false, \"error\": \"Invalid protocol: $protocol\"}"
        audit_log "$caller_container" "open" "$caller_container" "FAILED" "Invalid protocol"
        return 1
    fi
    
    if ! validate_port "$port"; then
        echo "{\"success\": false, \"error\": \"Invalid port: $port (must be 1024-65535)\"}"
        audit_log "$caller_container" "open" "$caller_container" "FAILED" "Invalid port"
        return 1
    fi
    
    if ! validate_source_ip "$source"; then
        echo "{\"success\": false, \"error\": \"Invalid source IP: $source\"}"
        audit_log "$caller_container" "open" "$caller_container" "FAILED" "Invalid source IP"
        return 1
    fi
    
    # Execute the script
    local output
    if output=$("${SCRIPTS_DIR}/modify_access.sh" \
        --container-name "$caller_container" \
        --action open \
        --protocol "$protocol" \
        --port "$port" \
        --source-ip "$source" \
        --force 2>&1); then
        
        echo "{\"success\": true, \"message\": \"Port opened successfully\", \"protocol\": \"$protocol\", \"port\": \"$port\"}"
        audit_log "$caller_container" "open" "$caller_container" "SUCCESS" "protocol=$protocol port=$port source=$source"
        return 0
    else
        echo "{\"success\": false, \"error\": \"Failed to open port\", \"details\": \"$output\"}"
        audit_log "$caller_container" "open" "$caller_container" "FAILED" "Script error: $output"
        return 1
    fi
}

# Execute close action
execute_close() {
    local caller_container="$1"
    local protocol="${2:-}"
    local port="${3:-}"
    
    # Validate inputs
    if [[ -z "$protocol" || -z "$port" ]]; then
        echo "{\"success\": false, \"error\": \"Protocol and port are required\"}"
        audit_log "$caller_container" "close" "$caller_container" "FAILED" "Missing parameters"
        return 1
    fi
    
    protocol=$(sanitize_input "$protocol")
    port=$(sanitize_input "$port")
    
    if ! validate_protocol "$protocol"; then
        echo "{\"success\": false, \"error\": \"Invalid protocol: $protocol\"}"
        audit_log "$caller_container" "close" "$caller_container" "FAILED" "Invalid protocol"
        return 1
    fi
    
    if ! validate_port "$port"; then
        echo "{\"success\": false, \"error\": \"Invalid port: $port\"}"
        audit_log "$caller_container" "close" "$caller_container" "FAILED" "Invalid port"
        return 1
    fi
    
    # Execute the script
    local output
    if output=$("${SCRIPTS_DIR}/modify_access.sh" \
        --container-name "$caller_container" \
        --action close \
        --protocol "$protocol" \
        --port "$port" \
        --force 2>&1); then
        
        echo "{\"success\": true, \"message\": \"Port closed successfully\", \"protocol\": \"$protocol\", \"port\": \"$port\"}"
        audit_log "$caller_container" "close" "$caller_container" "SUCCESS" "protocol=$protocol port=$port"
        return 0
    else
        echo "{\"success\": false, \"error\": \"Failed to close port\", \"details\": \"$output\"}"
        audit_log "$caller_container" "close" "$caller_container" "FAILED" "Script error: $output"
        return 1
    fi
}

# Execute list action
execute_list() {
    local caller_container="$1"
    
    local output
    if output=$("${SCRIPTS_DIR}/modify_access.sh" \
        --container-name "$caller_container" \
        --action list 2>&1); then
        
        # Parse output and return JSON (simplified)
        local mappings_file="/var/lib/container-ssh-mappings/${caller_container}_ports.json"
        if [[ -f "$mappings_file" ]]; then
            cat "$mappings_file"
        else
            echo "{\"ports\": []}"
        fi
        
        audit_log "$caller_container" "list" "$caller_container" "SUCCESS" ""
        return 0
    else
        echo "{\"success\": false, \"error\": \"Failed to list ports\"}"
        audit_log "$caller_container" "list" "$caller_container" "FAILED" "Script error"
        return 1
    fi
}

# Execute status action
execute_status() {
    local caller_container="$1"
    
    local output
    if output=$("${SCRIPTS_DIR}/modify_access.sh" \
        --container-name "$caller_container" \
        --action status 2>&1); then
        
        # Return simplified status as JSON
        local container_status=$(docker inspect -f '{{.State.Status}}' "$caller_container" 2>/dev/null || echo 'unknown')
        local container_ip=$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' "$caller_container" 2>/dev/null || echo 'unknown')
        
        echo "{\"success\": true, \"container\": \"$caller_container\", \"status\": \"$container_status\", \"ip\": \"$container_ip\"}"
        audit_log "$caller_container" "status" "$caller_container" "SUCCESS" ""
        return 0
    else
        echo "{\"success\": false, \"error\": \"Failed to get status\"}"
        audit_log "$caller_container" "status" "$caller_container" "FAILED" "Script error"
        return 1
    fi
}

# Handle request
handle_request() {
    local request="$1"
    local caller_pid="${2:-unknown}"
    
    log "Received request from PID $caller_pid: $request"
    
    # Parse JSON request
    local action=$(echo "$request" | jq -r '.action // empty')
    local protocol=$(echo "$request" | jq -r '.protocol // empty')
    local port=$(echo "$request" | jq -r '.port // empty')
    local source=$(echo "$request" | jq -r '.source // "*"')
    
    if [[ -z "$action" ]]; then
        echo "{\"success\": false, \"error\": \"Action is required\"}"
        return 1
    fi
    
    # Get caller's container
    local caller_container
    if ! caller_container=$(get_caller_container "$caller_pid"); then
        error "Could not determine caller container for PID $caller_pid"
        echo "{\"success\": false, \"error\": \"Unauthorized: Not called from a container\"}"
        audit_log "UNKNOWN" "$action" "PID:$caller_pid" "DENIED" "Not from container"
        return 1
    fi
    
    log "Request from container: $caller_container"
    
    # Check rate limit
    if ! check_rate_limit "$caller_container"; then
        echo "{\"success\": false, \"error\": \"Rate limit exceeded. Max $MAX_REQUESTS_PER_MINUTE requests per minute.\"}"
        audit_log "$caller_container" "$action" "$caller_container" "RATE_LIMITED" ""
        return 1
    fi
    
    # Execute action
    execute_action "$caller_container" "$action" "$protocol" "$port" "$source"
}

# Create socket directory
mkdir -p "$(dirname "$API_SOCKET")"
mkdir -p "$(dirname "$RATE_LIMIT_FILE")"
mkdir -p "$(dirname "$LOG_FILE")"
mkdir -p "$(dirname "$AUDIT_LOG")"

# Remove old socket
rm -f "$API_SOCKET"

# Start listening on Unix socket
log "Starting Container Access API on $API_SOCKET"

# Use socat or nc to listen on Unix socket
if command -v socat &>/dev/null; then
    while true; do
        socat UNIX-LISTEN:"$API_SOCKET",fork,mode=666 EXEC:"$0 --handle-connection",nofork
    done
elif [[ "$1" == "--handle-connection" ]]; then
    # Handle a single connection
    caller_pid=$(echo "$SOCAT_PEERADDR" | grep -oP '\d+' || echo "unknown")
    request=$(cat)
    handle_request "$request" "$caller_pid"
else
    error "socat is required but not installed"
    exit 1
fi