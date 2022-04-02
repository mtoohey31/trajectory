FROM rust:latest

WORKDIR /usr/src/trajectory-server
COPY . .

RUN apt-get update -y && apt install -y libpq-dev
RUN cargo install --path .

EXPOSE 8080

CMD ["trajectory-server"]
