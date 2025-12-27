FROM python:3.10-slim-bookworm

RUN pip install --no-cache-dir numpy pandas scipy pytest unittest-xml-reporting

# Ensure user-local binaries installed by agent setup are on PATH
ENV PATH="/root/.local/bin:${PATH}"

# 3. Set the working directory
WORKDIR /app


COPY . /task
WORKDIR /task
RUN rm -f /task/solution/solution.py && echo '# write your solution here' > /task/solution/solution.py
