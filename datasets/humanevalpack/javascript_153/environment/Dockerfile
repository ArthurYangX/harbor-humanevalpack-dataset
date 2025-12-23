FROM node:18-slim

RUN npm install -g js-md5 && npm cache clean --force
ENV NODE_PATH=/usr/local/lib/node_modules

# 3. 设置工作目录
WORKDIR /app

# 4. 切换到普通用户
# USER harbor


COPY . /task
WORKDIR /task