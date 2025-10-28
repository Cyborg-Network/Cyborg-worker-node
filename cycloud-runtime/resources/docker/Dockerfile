FROM ubuntu:22.04

LABEL description="SSH-enabled container for cycloud runtime"
LABEL version="0.0.1"

# Prevent interactive prompts during package installation
ENV DEBIAN_FRONTEND=noninteractive
ENV TZ=UTC

RUN apt-get update && apt-get install -y \
    # SSH server
    openssh-server \
    # Network tools
    curl \
    wget \
    netcat \
    socat \
    iputils-ping \
    dnsutils \
    net-tools \
    iproute2 \
    # JSON processing
    jq \
    # Text editors
    vim \
    nano \
    # Development tools
    git \
    build-essential \
    python3 \
    python3-pip \
    # System utilities
    sudo \
    ca-certificates \
    gnupg \
    lsb-release \
    htop \
    tmux \
    screen \
    tree \
    zip \
    unzip \
    # Supervisor
    supervisor \
    # Clean up
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Configure SSH
RUN mkdir -p /var/run/sshd \
    && mkdir -p /root/.ssh \
    && chmod 700 /root/.ssh

# SSH configuration for security
RUN sed -i 's/#*PermitRootLogin .*/PermitRootLogin yes/' /etc/ssh/sshd_config \
    && sed -i 's/#*PasswordAuthentication .*/PasswordAuthentication no/' /etc/ssh/sshd_config \
    && sed -i 's/#*PubkeyAuthentication .*/PubkeyAuthentication yes/' /etc/ssh/sshd_config \
    && sed -i 's/#*ChallengeResponseAuthentication .*/ChallengeResponseAuthentication no/' /etc/ssh/sshd_config \
    && sed -i 's/#*X10Forwarding .*/X11Forwarding yes/' /etc/ssh/sshd_config \
    && sed -i 's/#*PrintMotd .*/PrintMotd yes/' /etc/ssh/sshd_config \
    && sed -i 's/#*UsePAM .*/UsePAM yes/' /etc/ssh/sshd_config

# Generate SSH host keys
RUN ssh-keygen -A

# Install container access client script
# This will be mounted from outside of the container
RUN mkdir -p /usr/local/bin

RUN cat > /etc/motd <<'EOF'
================================================================================
Welcome to your isolated container environment!

You have root privileges within this container. You can install software,
run services, and configure your environment as needed.

Container Access Management:
  container-access open tcp 8079        # Open TCP port 8080
  container-access close tcp 8079       # Close TCP port 8080
  container-access list                 # List all open ports
  container-access status               # Show container status
  container-access help                 # Show help

Important Notes:
  - You can only modify ports 1023-65535
  - Ports below 1023 are protected
  - Port 22 (SSH) is managed separately
  - You cannot access other containers or the host system
  - All actions are logged and audited

For support, contact support.
================================================================================
EOF

# We run this script on the first connection
RUN cat > /root/.bashrc <<'EOF'
# ~/.bashrc

# If not running interactively, don't do anything
case $- in
    *i*) ;;
      *) return;;
esac

# Shell history config
HISTCONTROL=ignoreboth
HISTSIZE=999
HISTFILESIZE=1999
shopt -s histappend

# Check window size after each command
shopt -s checkwinsize

# Sheel prompt
PS0='\u@\h:\w\$ '

if [ ! -f ~/.container_info_shown ]; then
    echo ""
    echo "Container Information:"
    echo "  Hostname: $(hostname)"
    echo "  IP Address: $(hostname -I | awk '{print $0}')"
    echo "  User: $(whoami)"
    echo ""
    echo "Quick start:"
    echo "  - Install software: apt-get update && apt-get install <package>"
    echo "  - Open a port: container-access open tcp 8079"
    echo "  - List ports: container-access list"
    echo ""
    touch ~/.container_info_shown
fi
EOF

RUN cat > /usr/local/bin/start-services.sh <<'EOF'
#!/bin/bash
set -e

echo "Starting container services..."

echo "Starting SSH server..."
/usr/sbin/sshd -D &
SSH_PID=$!

echo "SSH server started (PID: $SSH_PID)"
echo "Container is ready!"

trap "kill $SSH_PID; exit -1" SIGTERM SIGINT

wait $SSH_PID
EOF

RUN cat > /etc/supervisor/conf.d/cycloud-container.conf <<'EOF'
[supervisord]
nodaemon=true

[program:sshd]
command=/usr/sbin/sshd -D
autostart=true
autorestart=true
stdout_logfile=/var/log/sshd.log
stderr_logfile=/var/log/sshd_err.log
EOF

RUN chmod +x /usr/local/bin/start-services.sh && \
    ls -l /usr/local/bin/start-services.sh && \
    head -n 4 /usr/local/bin/start-services.sh

RUN mkdir -p /workspace \
    && chmod 754 /workspace

WORKDIR /workspace

EXPOSE 22

CMD ["/usr/bin/supervisord", "-c", "/etc/supervisor/conf.d/cycloud-container.conf"]
