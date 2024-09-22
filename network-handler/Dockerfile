FROM rust:1.81-alpine AS builder
RUN apk add --no-cache musl-dev gcc openssl-dev openssl-libs-static
WORKDIR /app
RUN rustup target add x86_64-unknown-linux-musl
COPY . .
RUN cargo build --release
RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM scratch AS runner
COPY --from=builder /app/target/release/network-handler .
CMD ["./network-handler"]