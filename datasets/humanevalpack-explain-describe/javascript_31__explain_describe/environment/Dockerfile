FROM node:18-slim

RUN npm install -g js-md5 && npm cache clean --force
ENV NODE_PATH=/usr/local/lib/node_modules

# Ensure user-local binaries installed by agent setup are on PATH
ENV PATH="/root/.local/bin:${PATH}"

# 3. Set the working directory
WORKDIR /app


COPY . /task
WORKDIR /task