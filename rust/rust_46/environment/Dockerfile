FROM rust:latest



# 3. 设置工作目录
WORKDIR /app

# 4. 切换到普通用户
# USER harbor

WORKDIR /task
RUN cargo init --bin --name solution && cargo add regex rand md5 && cargo build --release

COPY . /task
WORKDIR /task