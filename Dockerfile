FROM lukemathwalker/cargo-chef:latest-rust-1.85 AS chef
WORKDIR /app
RUN apt update && apt install lld clang -y

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin shippr

FROM alpine/k8s:1.32.1 AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/shippr shippr