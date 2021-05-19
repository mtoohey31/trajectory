FROM rust as builder

WORKDIR /usr/src/trajectory-server
COPY . .

RUN apt-get update -y && apt install -y libpq-dev
RUN cargo install --path .

FROM rust:slim as runtime

COPY --from=builder /usr/local/cargo/bin/trajectory-server /usr/local/cargo/bin/trajectory-server

CMD ["trajectory-server"]
