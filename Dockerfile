FROM rust:1.71

WORKDIR /usr/src
COPY . . 

RUN cargo install --path xp-bot
EXPOSE 80

CMD ["cargo", "run", "--release"]