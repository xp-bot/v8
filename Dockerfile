FROM rust:latest

WORKDIR /usr/src
COPY . . 

RUN cargo install -p xp-bot
EXPOSE 80

CMD ["cargo", "build", "-p xp-bot", "--release"]
