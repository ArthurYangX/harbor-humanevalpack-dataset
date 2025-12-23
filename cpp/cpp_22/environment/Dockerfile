FROM gcc:latest

RUN bash -lc 'set -e; apt-get update -o Acquire::Retries=5; DEBIAN_FRONTEND=noninteractive apt-get install -o Acquire::Retries=5 -y --no-install-recommends g++ libboost-dev; rm -rf /var/lib/apt/lists/*'

# 3. 设置工作目录
WORKDIR /app

# 4. 切换到普通用户
# USER harbor


COPY . /task
WORKDIR /task