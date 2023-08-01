FROM rust:latest

RUN apt install make clang pkg-config libssl-dev
WORKDIR /usr/src/chatterfluxapi

COPY . .

# RUN cargo build

RUN cargo install --path .
EXPOSE 5001

CMD ["chatterfluxapi"]