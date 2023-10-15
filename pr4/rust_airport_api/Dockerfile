FROM rust:latest AS builder
COPY . .

RUN cargo build --release
CMD "/target/release/airport_api"
EXPOSE 8989
