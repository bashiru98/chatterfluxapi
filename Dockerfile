# Use a Debian-based image as the base image
FROM debian:latest

# Update the package lists, install the required packages, and clean up
RUN apt-get update && apt-get install -y \
    make \
    clang \
    pkg-config \
    libssl-dev \
    g++ \
    && rm -rf /var/lib/apt/lists/*


# Continue with your other Dockerfile instructions...

FROM rust:latest
WORKDIR /usr/src/chatterfluxapi

COPY . .

RUN cargo build --release

# RUN cargo install --path .
EXPOSE 5001

CMD ["chatterfluxapi"]