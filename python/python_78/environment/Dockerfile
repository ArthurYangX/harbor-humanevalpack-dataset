FROM python:3.10-slim

RUN pip install --no-cache-dir numpy pandas scipy pytest unittest-xml-reporting

# 3. 设置工作目录
WORKDIR /app

# 4. 切换到普通用户
# USER harbor


COPY . /task
WORKDIR /task