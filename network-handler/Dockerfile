FROM rust:1.81-alpine AS builder
RUN apk add --no-cache musl-dev gcc openssl-dev openssl-libs-static
WORKDIR /usr
RUN mkdir common-data-model
COPY common-data-model/. ./common-data-model
WORKDIR /usr/src/
RUN rustup target add x86_64-unknown-linux-musl
COPY network-handler/. .
RUN cargo build --release
RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM scratch AS runner
COPY --from=builder /usr/local/cargo/bin/network-handler .
CMD ["./network-handler"]