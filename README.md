# Venox Site - Full Stack Setup

This project is a full-stack web application with:
- **Frontend:** Vue 3 (Vite, Bun, TypeScript)
- **Backend:** Rust (Actix-web)
- **Dev Environments:** Nix flakes for reproducible builds
- **Containerization:** Docker & Docker Compose

---

## Prerequisites
- [Docker](https://www.docker.com/get-started)
- [Docker Compose](https://docs.docker.com/compose/)
- [Nix](https://nixos.org/download.html) (for local development/builds, optional for Docker usage)
- [Bun](https://bun.sh/) (for local frontend builds, optional for Docker usage)

---

## Quick Start (Docker Compose)

### 1. Build the Frontend Locally (Required for Docker Compose)
This ensures the `dist/` directory exists for the frontend container:

```sh
nix develop .# --command bun install
nix develop .# --command bun run build
```

### 2. Start the Full Stack App

```sh
docker-compose up --build
```

- **Frontend:** http://localhost:3000
- **Backend API:** http://localhost:9999

You can now reverse proxy the frontend (port 3000) and backend (port 9999) using Caddy or another proxy as needed.

---

## Project Structure

- `src/` - Frontend source code (Vue)
- `backend/` - Rust backend source code
- `Dockerfile` - Backend build and runtime image
- `frontend.Dockerfile` - Nginx static file server for frontend
- `docker-compose.yml` - Orchestrates both containers

---

## Development (Optional)

- **Frontend Dev:**
  ```sh
  nix develop .# --command bun install
  nix develop .# --command bun run dev
  # Runs on http://localhost:5173
  ```
- **Backend Dev:**
  ```sh
  cd backend
  nix develop .# --command cargo watch -x run
  # Runs on http://localhost:9999
  ```

---

## Notes
- The frontend is served by Nginx in the `frontend` container on port 3000.
- The backend API is served by the Rust binary in the `backend` container on port 9999.
- You can reverse proxy these services with Caddy, Nginx, or another proxy.
- All builds are reproducible via Nix flakes.

---

## License
MIT or as specified in this repository.
