## Overview
The Cyborg Worker Node is the one of the Cyborg Worker types, contributing compute resources to Cyborg Network, a decentralized compute platform designed to harness computational resources from distributed nodes around the world. By joining the network, users can either provide computational power to contribute to the network's infrastructure or consume computational resources for task execution.
## Usage 
#### Requirements
- A machine with internet access, running either Ubuntu 22 or 24
- A pre-funded account on the Cyborg Parachain
- A Pinata IPFS API key and secret
#### Installation

##### Method 1: Docker
1. Clone the GitHub repository for the Cyborg Worker Node and navigate into it:
```
git clone
```
##### Method 2: Via Installation Script
1. Download the installation script from: https://github.com/Cyborg-Network/Cyborg-worker-node/blob/tom/standalone-worker-subxt/cyborg-worker-node-installer.sh
2. In the terminal, navigate to the location in which the script was saved, make the script executable and run it with elevated privileges: 
```
   cd $SCRIPT_LOCATION/
   sudo chmod +x cyborg-worker-node-installer.sh
   sudo bash cyborg-worker-node-installer.sh
```
3. Follow the instructions of the script. When prompted for the URL of the parachain the Cyborg Worker Node should connect to, provide the following url to register the worker on the Cyborg testnet: `wss://fraa-dancebox-3131-rpc.a.dancebox.tanssi.network`. Please note that quotation marks are NOT required for the account seed.

The script will perform the same actions that are outlined in Method 2 (except for the fact, that it will not clone the github repository, but download a binary containing the Cyborg Worker Node).
##### Method 3: Compile From Source
##### Installation Requirements
1. Have the rust toolchain installed
2. Have nightly features enabled
###### Steps to install
1. Clone the Cyborg Worker Node repository, navigate to it and compile the code: 
```
git clone https://github.com/Cyborg-Network/Cyborg-worker-node.git`
cd Cyborg-worker-node
cargo build --release
```
2. Download the Cyborg Agent: 
   https://server.cyborgnetwork.io:8080/assets/cyborg-agent
   The Agent is required for the Cyborg Worker Node and Cyborg Connect to be able to communicate off-chain data.
3. Make the `cyborg-agent` and `cyborg-worker-node` executable and move them to  `/usr/local/bin`:
```
chmod +x cyborg-agent
chmod +x cyborg-worker-node
sudo mv cyborg-agent /usr/local/bin/
sudo mv cyborg-worker-node /usr/local/bin/
```
4. Create a user that manages the Cyborg executables:
```
sudo useradd -r -s /bin/false cyborg-user
```
5. Create the directories where the files managed by the cyborg executables will exist and give the newly created user the permissions required to manage these directories:
```
sudo mkdir -p /var/lib/cyborg/worker-node/packages
sudo mkdir -p /var/lib/cyborg/worker-node/config
sudo mkdir -p /var/lib/cyborg/worker-node/logs
sudo chown -R cyborg-user:cyborg-user /var/lib/cyborg
sudo chmod -R 700 /var/lib/cyborg
```
6. Register the worker (replace the variables with your data):
```
sudo cyborg-worker-node registration --parachain-url "$PARACHAIN_URL" --account-seed "$ACCOUNT_SEED" --ipfs-url "$IPFS_URL" --ipfs-api-key "$IPFS_KEY" --ipfs-api-secret "$IPFS_SECRET"
```
7. Create a DBus configuration file to set up an internal communication channel between the Cyborg Worker Node and the Cyborg Agent:
```
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
```
8. Create `systemd` configuration files for the Cyborg Agent and the Cyborg Worker Node, so that the executables don't need to be handled manually. Note that in the commands the variables in `Enviroment` will need to be replaced, but NOT the others.
```
sudo bash -c "cat > /etc/systemd/system/cyborg-worker-node.service" << EOL
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



sudo bash -c "cat > /etc/systemd/system/cyborg-agent.service" << EOL
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
```
9. Reload the systems daemons, and start the newly created `systemd` services. This will prompt the Cyborg Worker Node to start listening for tasks:
```
sudo systemctl daemon-reload
sudo systemctl enable cyborg-worker-node
sudo systemctl enable cyborg-agent
sudo systemctl start cyborg-worker-node
sudo systemctl start cyborg-agent
```
10. Open ports `8080` and `8081` in your firewall. The commands for this depend on your firewall, so if there is uncertainty please consult the documentation of your firewall. These ports need to be open because the agent needs to:
	1. Communicate with Cyborg Connect to send usage data and logs
	2. Respond to requests from the Cyborg Oracle Feeder which will occasionally query the Cyborg Agent to provide an information on the health and uptime of the Cyborg Worker Node

Congratulations, your machine is now a Cyborg Worker Node! It will listen to the Cyborg Parachain, execute tasks that were assigned to it and verify the results of other Nodes.

## Testing
##### Requirements
1. Have the rust toolchain installed
2. Have nightly features enabled
##### Steps to Test
1. Clone the Cyborg Worker Node repository, navigate to it and run the unit tests: 
```
git clone https://github.com/Cyborg-Network/Cyborg-worker-node.git`
cd Cyborg-worker-node
cargo test
```

