FROM rust:latest

WORKDIR /usr/src/chatterfluxapi

COPY . .

# RUN cargo build

RUN cargo install --path .
EXPOSE 5001

CMD ["chatterfluxapi"]