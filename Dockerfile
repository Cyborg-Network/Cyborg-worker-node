FROM ubuntu:24.04

LABEL maintainer="tom@cyborgnetwork.io"
LABEL description="Demo container for the Cyborg Worker Node"

# For testnet: wss://fraa-dancebox-3131-rpc.a.dancebox.tanssi.network
# For local zombienet: ws://127.0.0.1:9988
ENV PARACHAIN_URL=
ENV ACCOUNT_SEED=
ENV IPFS_URL=
ENV IPFS_KEY=
ENV IPFS_SECRET=

WORKDIR /root

RUN apt-get update && apt-get install -y \
    sudo \
    curl \
    git \
    wget \
    bash \
    && apt-get clean

EXPOSE 8080 8081

COPY ./cyborg-worker-node-installer.sh /tmp/installer.sh

RUN bash /tmp/installer.sh
