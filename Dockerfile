FROM rust:1.91-bookworm@sha256:8fed34f697cc63b2c9bb92233b4c078667786834d94dd51880cd0184285eefcf AS builder

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release


FROM gcr.io/distroless/cc-debian12@sha256:0000f9dc0290f8eaf0ecceafbc35e803649087ea7879570fbc78372df7ac649b

COPY --from=builder /usr/src/app/target/release/simple-line-notice /usr/local/bin/simple-line-notice

CMD ["/usr/local/bin/simple-line-notice"]
