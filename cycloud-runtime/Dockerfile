FROM ubuntu:22.04

RUN apt-get update && apt-get install -y \
    sudo \
    bash \
    git \
    curl \
    vim \
    python3 \
    python3-pip \
    && rm -rf /var/lib/apt/lists/*

ARG USER=developer
ARG UID=1000
ARG GID=1000
RUN groupadd -g $GID $USER && \
    useradd -m -u $UID -g $GID -s /bin/bash $USER && \
    echo "$USER ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

USER $USER
WORKDIR /home/$USER

CMD ["/bin/bash"]
