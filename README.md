# Cyborg Worker Node CLI

A standalone command-line application for managing off-chain worker nodes in the Cyborg network.

## Overview

This command-line application enable workers to register, start sessions (mining), and interact with the network efficiently.

The Cyborg Worker Node CLI provides functionality, including:

* Worker Registration: Register a worker node with a specified Cyborg Parachain endopoint and Account Id.
* Session Management: Start a session (mining)

### Terminology

* **Mining:** Use the Cyborg Worker Node CLI to execute and complete task assignment.

## Commands

The following command are available

**1. Registration**

* **Usage:**

  ```bash
  cyborg-worker registration --api-url <API_URL> --account-id <ACCOUNT_ID>

  ```

**Start**

* **Usage:**

  ```bash
  cyborg-worker start --api-url <API_URL> --ipfs-url <IFS_URL>


License: Apache-2.0
