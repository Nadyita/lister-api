# Build stage - using Rust with musl support
FROM rust:1.90-alpine AS builder

# Install musl development tools
RUN apk add --no-cache musl-dev

# Set the working directory
WORKDIR /app

# Copy dependency manifests
COPY Cargo.toml Cargo.lock ./

# Create a dummy main to build dependencies (caching layer)
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release --target x86_64-unknown-linux-musl && \
    rm -rf src

# Copy the actual source code
COPY src ./src

# Build the actual application (dependencies are cached)
RUN touch src/main.rs && \
    cargo build --release --target x86_64-unknown-linux-musl && \
    strip target/x86_64-unknown-linux-musl/release/ultimatelister-api

# Runtime stage - minimal scratch image
FROM scratch

# Copy the statically linked binary from builder
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/ultimatelister-api /ultimatelister-api

# Set default environment variables for container deployment
ENV HOST=0.0.0.0
ENV PORT=8080

# Expose the port
EXPOSE 8080

# Set the entrypoint
ENTRYPOINT ["/ultimatelister-api"]

