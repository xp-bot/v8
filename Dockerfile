FROM rust:latest

WORKDIR /usr/src
COPY . . 

RUN cargo install --path xp-bot
EXPOSE 80

CMD ["cargo", "build", "--release", "--path xp-bot"]
