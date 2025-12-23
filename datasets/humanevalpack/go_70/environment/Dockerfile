FROM golang:1.22



# 3. 设置工作目录
WORKDIR /app

# 4. 切换到普通用户
# USER harbor

WORKDIR /task
RUN go mod init task && go get github.com/stretchr/testify/assert

COPY . /task
WORKDIR /task