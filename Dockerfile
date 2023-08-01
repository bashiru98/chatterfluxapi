# # Use an Ubuntu-based image as the base image
# FROM ubuntu:latest

# # Update the package lists, install the libclang-dev package, and clean up
# RUN apt-get update && apt-get install -y libclang && rm -rf /var/lib/apt/lists/*


# # Continue with your other Dockerfile instructions...

# FROM rust:latest
# WORKDIR /usr/src/chatterfluxapi

# COPY . .

# RUN cargo build --release

# # RUN cargo install --path .
# EXPOSE 5001

# CMD ["chatterfluxapi"]


# Build stage
FROM rust:1.69-buster as builder

WORKDIR /app


# Copy the source code
COPY . .

# Build the application
RUN cargo build --release


# Production stage
FROM debian:buster-slim

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/chatterfluxapi .

EXPOSE 5001

CMD ["./chatterfluxapi"]