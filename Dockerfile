FROM rust:1
WORKDIR /app
RUN apt-get update && \
    apt-get install -y --quiet default-mysql-client && \
    rm -rf /var/lib/apt/lists/*
COPY . .
RUN cargo install --path .

CMD ["migration"]