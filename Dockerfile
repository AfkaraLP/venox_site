FROM oven/bun:1.3.14 AS builder

RUN apt-get update && apt-get install -y --no-install-recommends procps && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .

RUN bun install
RUN bun run build

FROM nginx:stable-alpine

COPY --from=builder /app/dist /usr/share/nginx/html

COPY frontend.nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 9998
CMD ["nginx", "-g", "daemon off;"]
