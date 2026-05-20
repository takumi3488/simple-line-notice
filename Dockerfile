FROM rust:1.95-bookworm@sha256:7c0ab4af777bc8049a5051a4a89f152ac0ce8276f8aa4ec0e1d7670bc74ad597 AS builder

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release


FROM gcr.io/distroless/cc-debian12@sha256:aa0b7af67fa8211751ea6e00baa8373ba56cc1417ffc986ec9619bd0e1556b56

COPY --from=builder /usr/src/app/target/release/simple-line-notice /usr/local/bin/simple-line-notice

CMD ["/usr/local/bin/simple-line-notice"]
