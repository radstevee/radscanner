FROM rust:1.67 as builder
WORKDIR /usr/src/radscanner
COPY . .
RUN cargo build --release
CMD ["./target/release/radscanner"]