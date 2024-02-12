FROM rust:1.75.0-alpine AS builder
RUN apk add --no-cache musl-dev
WORKDIR /app
COPY . .
RUN cargo build --release --locked

FROM alpine:3.18
COPY --from=builder /app/target/release/crebito /usr/local/bin/crebito
CMD ["crebito"]
