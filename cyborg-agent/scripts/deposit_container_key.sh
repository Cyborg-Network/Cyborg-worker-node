#!/bin/bash

error() {
    echo -e "\033[31m$1\033[0m" >&2
    exit 1
}

CONTAINER="$1"
PUBLIC_KEY="$2"

[ -z "$CONTAINER" ] && error "Usage: $0 <container> <public_key>"
[ -z "$PUBLIC_KEY" ] && error "Public key required"

if [[ ! -x "$(command -v docker)" ]]; then 
    error "Docker is not installed"
fi

if [[ "$(docker inspect -f '{{.State.Running}}' $CONTAINER)" != "true" ]]; then
    error "Container $CONTAINER is not running"
fi

docker exec "$CONTAINER" mkdir -p /root/.ssh
docker exec "$CONTAINER" chmod 700 /root/.ssh
docker exec "$CONTAINER" sh -c "echo '$PUBLIC_KEY' > /root/.ssh/authorized_keys"
docker exec "$CONTAINER" chmod 600 /root/.ssh/authorized_keys

echo "SUCCESS"