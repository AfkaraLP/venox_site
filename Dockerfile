FROM oven/bun:1.1 AS builder

WORKDIR /app
COPY . .

RUN bun install
RUN bun run build

FROM nginx:stable-alpine

COPY --from=builder /app/dist /usr/share/nginx/html

COPY frontend.nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 9998
CMD ["nginx", "-g", "daemon off;"]
