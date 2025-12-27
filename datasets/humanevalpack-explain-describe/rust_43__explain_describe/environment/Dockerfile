FROM rust:latest



# Ensure user-local binaries installed by agent setup are on PATH
ENV PATH="/root/.local/bin:${PATH}"

# 3. Set the working directory
WORKDIR /app

WORKDIR /task
RUN cargo init --bin --name solution && cargo add regex rand md5 && cargo build --release

COPY . /task
WORKDIR /task