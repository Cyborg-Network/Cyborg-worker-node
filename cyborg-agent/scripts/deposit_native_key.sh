#!/bin/bash

error() {
    echo -e "\033[31m$1\033[0m" >&2
    exit 1
}

ACTIVE_USER="$1"
PUBLIC_KEY="$2"

[ -z "$ACTIVE_USER" ] && error "Usage: $0 <active_user> <public_key>"
[ -z "$PUBLIC_KEY" ] && error "Public key required"


if ! id "$ACTIVE_USER" &>/dev/null; then
    echo "User '$ACTIVE_USER' does not exist, creating..."
    sudo useradd "$ACTIVE_USER"
fi

SSH_DIR="/home/$ACTIVE_USER/.ssh"
mkdir -p "$SSH_DIR"
echo "$PUBLIC_KEY" > "$SSH_DIR/authorized_keys"
chmod 700 "$SSH_DIR"
chmod 600 "$SSH_DIR/authorized_keys"
chown -R "$ACTIVE_USER:$ACTIVE_USER" "$SSH_DIR"

echo "SUCCESS"
