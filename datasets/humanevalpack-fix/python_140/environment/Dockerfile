FROM python:3.10-slim-bookworm

RUN pip install --no-cache-dir numpy pandas scipy pytest unittest-xml-reporting

# Ensure user-local binaries installed by agent setup are on PATH
ENV PATH="/root/.local/bin:${PATH}"

# 3. Set the working directory
WORKDIR /app

# 4. Switch to a non-root user (optional)
# USER harbor


COPY . /task
WORKDIR /task