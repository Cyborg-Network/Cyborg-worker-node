FROM ubuntu:24.04

LABEL maintainer="tom@cyborgnetwork.io"
LABEL description="Demo container for the Cyborg Worker Node"

ENV PARACHAIN_URL=ws://127.0.0.1:9988
ENV CYBORG_WORKER_NODE_IPFS_API_URL=https://fuchsia-academic-stoat-866.mypinata.cloud
ENV CYBORG_WORKER_NODE_IPFS_API_KEY=21021fa56da65b48c301
ENV CYBORG_WORKER_NODE_IPFS_API_SECRET=6df0a896d2c37606f53ae39f02333484be86d429a898e7c38fb8e4f67da16cb2
#ENV CYBORG_WORKER_NODE_TEST_IP=

WORKDIR /root

RUN apt-get update && apt-get install -y \
    sudo \
    curl \
    git \
    wget \
    bash \
    dbus \
    supervisor \
    && apt-get clean

RUN useradd -r -s /bin/false cyborg-user

RUN mkdir -p /var/lib/cyborg/worker-node/{packages,config,logs} \
    && chown -R cyborg-user:cyborg-user /var/lib/cyborg

COPY docker-installer.sh /install.sh
RUN chmod +x /install.sh && /install.sh

COPY supervisord.conf /etc/supervisor/conf.d/supervisord.config

EXPOSE 8080 8081

#/usr/local/bin/cyborg-worker-node registration --parachain-url "$PARACHAIN_URL" --account-seed "$ACCOUNT_SEED" --ipfs-url "$CYBORG_WORKER_NODE_IPFS_API_URL" --ipfs-api-key "$CYBORG_WORKER_NODE_IPFS_API_KEY" --ipfs-api-secret "$CYBORG_WORKER_NODE_IPFS_API_SECRET"

CMD ["/usr/bin/supervisord", "-c", "/etc/supervisor/conf.d/supervisord.config"]
