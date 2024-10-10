FROM rust:1.81-slim-bullseye as builder

# Build project
WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .

# Setup run environment
FROM debian:bullseye-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/labeler /usr/local/bin/labeler
ENTRYPOINT ["/usr/local/bin/labeler"]
