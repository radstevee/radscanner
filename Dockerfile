FROM rust:1.68

WORKDIR /usr/src/radscanner
COPY . .

RUN cargo build --release

CMD ["./target/release/radscanner"]
