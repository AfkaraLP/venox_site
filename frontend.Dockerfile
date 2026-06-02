# Stage 1: Build with Nix Flakes
FROM nixos/nix:2.18.1 as builder
ENV NIX_CONFIG 'experimental-features = nix-command flakes'
WORKDIR /workspace
COPY . .
RUN nix develop .# --command bun install
RUN nix develop .# --command bun run build

# Stage 2: Nginx static file server
FROM nginx:alpine
COPY --from=builder /workspace/dist /usr/share/nginx/html
EXPOSE 80 