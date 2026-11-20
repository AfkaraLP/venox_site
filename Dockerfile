# oven/bun:1.3.14 segfaults on this kernel (SIGTRAP/139 at `bun run build`),
# so the frontend is built on the host with `npx vite build` instead.
# Run that BEFORE `docker compose build frontend`.
FROM alpine:3.20 AS builder

COPY dist /dist

FROM nginx:stable-alpine

COPY --from=builder /dist /usr/share/nginx/html

COPY frontend.nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 9998
CMD ["nginx", "-g", "daemon off;"]
