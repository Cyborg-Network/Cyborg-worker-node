#!/bin/bash

error() {
    echo -e "\033[31m$1\033[0m" >&2
    exit 1
}

VM_NAME="$1"
ACTIVE_USER="$2"
PUBLIC_KEY="$3"

[ -z "$VM_NAME" ] && error "Usage: $0 <vm_name> <active_user> <public_key>"
[ -z "$ACTIVE_USER" ] && error "Usage: $0 <vm-name> <active_user> <public_key>"
[ -z "$PUBLIC_KEY" ] && error "Usage: $0 <vm-name> <active_user> <public_key>"


if ! id "$ACTIVE_USER" &>/dev/null; then
    echo "User '$ACTIVE_USER' does not exist, creating..."
    sudo useradd "$ACTIVE_USER"
fi

virt-customize -d "$VM_NAME" --run-command \
    "mkdir -p /home/$USERNAME/.ssh && echo '$PUB_KEY' >> /home/$USERNAME/.ssh/authorized_keys && chmod 700 /home/$USERNAME/.ssh && chmod 600 /home/$USERNAME/.ssh/authorized_keys && chown -R $USERNAME:$USERNAME /home/$USERNAME/.ssh"

if [ $? -ne 0 ]; then
    echo "Failed to deposit SSH keys"
    exit 1
fi

echo "SSH keys deposited successfully for user: $USERNAME"
