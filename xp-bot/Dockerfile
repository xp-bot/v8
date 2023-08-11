FROM rust:latest

WORKDIR /usr/src
COPY . . 

RUN cargo install --path .
EXPOSE 80

CMD ["cargo", "build", "--release"]
