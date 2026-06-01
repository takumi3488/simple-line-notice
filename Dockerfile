FROM rust:1.96-bookworm@sha256:13c186980fa33cc12759b429662a1322939dbe697484b7c33b47dd2698d28460 AS builder

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release


FROM gcr.io/distroless/cc-debian12@sha256:aa0b7af67fa8211751ea6e00baa8373ba56cc1417ffc986ec9619bd0e1556b56

COPY --from=builder /usr/src/app/target/release/simple-line-notice /usr/local/bin/simple-line-notice

CMD ["/usr/local/bin/simple-line-notice"]
