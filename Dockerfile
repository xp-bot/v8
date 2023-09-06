FROM rust:latest

WORKDIR /usr/src
COPY . . 

RUN cargo build --release
EXPOSE 80

CMD ["./target/release/xp-bot"]