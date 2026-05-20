FROM rust:1.95-bookworm@sha256:536e16f10a79cbf0347f5da6994caaf0dc81863cadcb503e07ec6b5fca02f0b6 AS builder

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release


FROM gcr.io/distroless/cc-debian12@sha256:aa0b7af67fa8211751ea6e00baa8373ba56cc1417ffc986ec9619bd0e1556b56

COPY --from=builder /usr/src/app/target/release/simple-line-notice /usr/local/bin/simple-line-notice

CMD ["/usr/local/bin/simple-line-notice"]
