
FROM rust:latest AS builder

RUN apt-get update && apt-get install -y ca-certificates

WORKDIR /usr/src/ralert

COPY . .

RUN cargo build --release


FROM debian:bookworm-slim AS runtime

WORKDIR /app

COPY --from=builder /usr/src/ralert/target/release/ralert /app/ralert


CMD ["./ralert"]
