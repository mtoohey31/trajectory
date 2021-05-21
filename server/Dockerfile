FROM rust

WORKDIR /usr/src/trajectory-server
COPY . .

RUN apt-get update -y && apt install -y libpq-dev
RUN cargo install --path .

CMD ["trajectory-server"]
