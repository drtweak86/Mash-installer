# Dockerfile for MASH-installer
# Multi-stage build for creating a containerized installer

# Stage 1: Build the Rust application
FROM rust:1.93 as builder

WORKDIR /app

# Copy cargo and build files
COPY Cargo.toml Cargo.lock ./
COPY installer-cli/Cargo.toml installer-cli/
COPY installer-core/Cargo.toml installer-core/
COPY installer-arch/Cargo.toml installer-arch/
COPY installer-debian/Cargo.toml installer-debian/
COPY installer-fedora/Cargo.toml installer-fedora/

# Build dependencies
RUN cargo fetch --target x86_64-unknown-linux-gnu

# Copy source files
COPY src/ src/
COPY installer-cli/src/ installer-cli/src/
COPY installer-core/src/ installer-core/src/
COPY installer-arch/src/ installer-arch/src/
COPY installer-debian/src/ installer-debian/src/
COPY installer-fedora/src/ installer-fedora/src/

# Build release binary
RUN cargo build --release --bin mash-setup

# Stage 2: Create runtime image
FROM alpine:latest as runtime

RUN apk add --no-cache \
    bash \
    curl \
    ca-certificates \
    libgcc \
    musl-locales \
    tzdata \
    && update-ca-certificates

WORKDIR /root/

# Copy the binary from builder
COPY --from=builder /app/target/release/mash-setup /usr/local/bin/mash-setup

# Copy install script
COPY install.sh /usr/local/bin/install-mash

# Make install script executable
RUN chmod +x /usr/local/bin/install-mash

# Set entrypoint
ENTRYPOINT ["/usr/local/bin/mash-setup"]

# Default command (can be overridden)
CMD ["--help"]
