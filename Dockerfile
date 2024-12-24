# Use the official Rust image as the base image
FROM rust:latest AS builder

# Set the working directory
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

# Build the project
RUN cargo build --release

# Use a minimal base image for the final stage
FROM debian:bullseye-slim

# Set the working directory
WORKDIR /usr/src/app

# Install necessary runtime dependencies
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/tcp_communication .

# Verify the binary exists
RUN ls -la /usr/src/app/tcp_communication

# Set the entrypoint to the compiled binary
ENTRYPOINT ["./tcp_communication"]