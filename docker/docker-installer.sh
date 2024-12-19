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

# Create directories and setup permissions
sudo mkdir -p /var/lib/cyborg/worker-node/{packages,config,logs}
sudo chown -R cyborg-user:cyborg-user /var/lib/cyborg

# DBus configuration for worker node
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
