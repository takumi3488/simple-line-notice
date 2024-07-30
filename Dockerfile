FROM rust:1.80-bookworm AS builder

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release


FROM gcr.io/distroless/cc-debian12

COPY --from=builder /usr/src/app/target/release/simple-line-notice /usr/local/bin/simple-line-notice

CMD ["/usr/local/bin/simple-line-notice"]
