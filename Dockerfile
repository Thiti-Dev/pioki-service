FROM rust:latest

WORKDIR /usr/src/pioki-app

COPY . .

RUN apt-get update && apt-get install -y musl-dev

RUN cargo build --release --target-dir ./build

CMD ["./build/release/authentication"]