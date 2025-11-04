FROM rust:alpine AS builder
RUN apk add --no-cache musl-dev
WORKDIR /app
COPY . .
RUN cargo build --release

FROM alpine:latest
WORKDIR /app
RUN mkdir -p /app/data
COPY --from=builder /app/target/release/ralert .
COPY --from=builder /app/migrations ./migrations
EXPOSE 8003
CMD ["./ralert"]
