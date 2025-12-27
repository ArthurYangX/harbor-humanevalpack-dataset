FROM eclipse-temurin:17-jdk



# Ensure user-local binaries installed by agent setup are on PATH
ENV PATH="/root/.local/bin:${PATH}"

# 3. Set the working directory
WORKDIR /app


COPY . /task
WORKDIR /task