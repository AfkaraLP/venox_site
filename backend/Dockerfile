FROM rust:latest

WORKDIR /app

# Build dependencies first with a dummy main so cargo compiles are cached
# unless Cargo.lock / dependencies actually change
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release && rm -rf src

COPY . .
RUN touch src/main.rs && cargo build --release

CMD ["./target/release/backend"]
EXPOSE 9999
