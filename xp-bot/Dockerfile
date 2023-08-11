FROM rust:latest

WORKDIR /usr/src
COPY . . 

RUN cargo install
EXPOSE 80

CMD ["cargo", "run", "--release"]
