FROM eclipse-temurin:17-jdk



# Ensure user-local binaries installed by agent setup are on PATH
ENV PATH="/root/.local/bin:${PATH}"

# 3. Set the working directory
WORKDIR /app

# 4. Switch to a non-root user (optional)
# USER harbor


COPY . /task
WORKDIR /task