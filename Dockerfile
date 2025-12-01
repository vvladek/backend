# Stage 1: build
FROM rust:1.91.1 as builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

# Stage 2: runtime
FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /app/target/release/backend .
COPY .env ./
CMD ["./backend"]