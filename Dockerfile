FROM rust:1.93-bookworm@sha256:812df42b4a866cf7165934691a0a89061281679a145b857dc679be8132e709b9 AS builder

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release


FROM gcr.io/distroless/cc-debian12@sha256:66d87e170bc2c5e2b8cf853501141c3c55b4e502b8677595c57534df54a68cc5

COPY --from=builder /usr/src/app/target/release/simple-line-notice /usr/local/bin/simple-line-notice

CMD ["/usr/local/bin/simple-line-notice"]
