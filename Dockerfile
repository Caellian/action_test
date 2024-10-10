# Use an official Rust image as the base
FROM rust:1.81-slim as builder

# Create app directory and copy over the project
WORKDIR /usr/src/app
COPY . .

# Build the Rust project
RUN cargo build --release

# Use a smaller base image for the final stage
FROM debian:buster-slim

# Copy the Rust binary from the build stage
COPY --from=builder /usr/src/app/target/release/labeler /usr/local/bin/labeler

# Set the entrypoint to run the Rust binary
ENTRYPOINT ["/usr/local/bin/labeler"]
