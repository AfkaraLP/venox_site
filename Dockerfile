# Stage 1: Build with Nix Flakes
FROM nixos/nix:2.18.1 as builder

# Enable flakes and experimental features
ENV NIX_CONFIG 'experimental-features = nix-command flakes'

# Copy the whole repo
WORKDIR /workspace
COPY . .

# Build backend (Rust)
WORKDIR /workspace/backend
RUN nix develop .# --command cargo build --release

# Build frontend (Bun)
WORKDIR /workspace
RUN nix develop .# --command bun install
RUN nix develop .# --command bun run build

# Stage 2: Minimal runtime image
FROM debian:bookworm-slim

# Install minimal dependencies (e.g., for SSL if needed)
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy backend binary
COPY --from=builder /workspace/backend/target/release/backend /usr/local/bin/backend

# Copy frontend static files
COPY --from=builder /workspace/dist /srv/frontend

# Expose ports (adjust as needed)
EXPOSE 9999

# Set environment variables if needed
ENV FRONTEND_DIST=/srv/frontend

# Start the backend (assumes it serves static files from $FRONTEND_DIST)
CMD ["/usr/local/bin/backend"] 