# Dockerfile for rust-form
FROM nixos/nix:latest AS builder

# Copy source code
WORKDIR /app
COPY . .

# Install git (needed for flake operations)
RUN nix-env -iA nixpkgs.git

# Enable nix flakes
RUN echo "experimental-features = nix-command flakes" >> /etc/nix/nix.conf

# Build the application using Nix
RUN nix build

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y \
        ca-certificates \
        sqlite3 \
        curl \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -r -s /bin/false rustform

# Copy the binary from builder
COPY --from=builder /app/result/bin/rustform /usr/local/bin/rustform

# Make binary executable
RUN chmod +x /usr/local/bin/rustform

# Create directories
RUN mkdir -p /app/data /app/config && \
    chown -R rustform:rustform /app

# Switch to app user
USER rustform

# Set working directory
WORKDIR /app

# Set environment variables
ENV RUST_LOG=info
ENV DATABASE_URL=sqlite:/app/data/rustform.db

# Expose port (adjust as needed)
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD rustform --version || exit 1

# Default command
CMD ["rustform", "--help"]