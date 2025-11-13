
<h1 align="center">
  <br>
  <a href="http://www.cyborgnetwork.io"><img src="https://github.com/user-attachments/assets/16067543-6de0-4f62-9a03-d82da38f001a" width="200"></a>
  <br>
  Cyborg Agent
  <br>
</h1>

<h4 align="center">An agent that aggregates usage metrics, specs and logs on a <a href="https://github.com/Cyborg-Network/Cyborg-worker-node" target="_blank">Cyborg Miner</a> and streams them to requesting <a href="https://github.com/Cyborg-Network/cyborg-connect" target="_blank">Cyborg Connect</a> clients.</h4>

## Key Features

* Metric Streaming - Streams usage metrics of a <a href="https://github.com/Cyborg-Network/Cyborg-worker-node" target="_blank">Cyborg Miners</a>.
* Spec Detection - Detects the specs of a <a href="https://github.com/Cyborg-Network/Cyborg-worker-node" target="_blank">Cyborg Miners</a>.
* Log Streaming - Streams log messages from a <a href="https://github.com/Cyborg-Network/Cyborg-worker-node" target="_blank">Cyborg Miners</a>.
* Health Checking - Checks the health of a <a href="https://github.com/Cyborg-Network/Cyborg-worker-node" target="_blank">Cyborg Miner</a>.


## How To Use

To build and run this application you will need to have the rust toolchain installed:

```bash
# Clone this repository
$ git clone https://github.com/Cyborg-Network/Cyborg-smart-client

# Cd into the repository
$ cd Cyborg-smart-client

# Build the project
$ cargo build --release

# Run the app
$ ./target/release/Cyborg-smart-client
```
