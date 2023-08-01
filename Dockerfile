# Use a Debian-based image as the base image
FROM debian:latest

# Update the package lists for upgrades and new package installations
RUN apt-get update

# Install the required packages
RUN apt-get install -y make clang pkg-config libssl-dev

FROM rust:latest
WORKDIR /usr/src/chatterfluxapi

COPY . .

# RUN cargo build

RUN cargo install --path .
EXPOSE 5001

CMD ["chatterfluxapi"]