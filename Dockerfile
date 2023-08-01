# Use an Ubuntu-based image as the base image
FROM ubuntu:latest

# Update the package lists, install the libclang-dev package, and clean up
RUN apt-get update && apt-get install -y libclang-dev && rm -rf /var/lib/apt/lists/*


# Continue with your other Dockerfile instructions...

FROM rust:latest
WORKDIR /usr/src/chatterfluxapi

COPY . .

RUN cargo build --release

# RUN cargo install --path .
EXPOSE 5001

CMD ["chatterfluxapi"]