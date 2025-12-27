FROM gcc:latest

RUN bash -lc 'set -e; apt-get update -o Acquire::Retries=5; DEBIAN_FRONTEND=noninteractive apt-get install -o Acquire::Retries=5 -y --no-install-recommends g++; rm -rf /var/lib/apt/lists/*'

# Ensure user-local binaries installed by agent setup are on PATH
ENV PATH="/root/.local/bin:${PATH}"

# 3. Set the working directory
WORKDIR /app

# 4. Switch to a non-root user (optional)
# USER harbor


COPY . /task
WORKDIR /task