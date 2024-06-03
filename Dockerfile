# Stage 1: Build the Rust application
FROM rust:1.64 as builder

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files first, to leverage Docker cache
COPY Cargo.toml Cargo.lock ./

# Create an empty main.rs and build the dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# This step will cache the dependencies
RUN cargo build --release

# Now copy the source code and build the project
COPY . .

# Build the Rust project
RUN cargo build --release

# Stage 2: Create a smaller image for running the application
FROM debian:buster-slim

# Install dependencies for running the application
RUN apt-get update && apt-get install -y \
    libssl1.1 \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Copy the build artifact from the builder stage
COPY --from=builder /usr/src/app/target/release/lab /usr/local/bin/lab

# Set the startup command to run the binary
CMD ["lab"]
