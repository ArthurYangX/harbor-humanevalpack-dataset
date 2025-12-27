FROM golang:1.22



# Ensure user-local binaries installed by agent setup are on PATH
ENV PATH="/root/.local/bin:${PATH}"

# 3. Set the working directory
WORKDIR /app

# 4. Switch to a non-root user (optional)
# USER harbor

WORKDIR /task
RUN go mod init task && go get github.com/stretchr/testify/assert

COPY . /task
WORKDIR /task