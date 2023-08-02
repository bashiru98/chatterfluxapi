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
# FROM rust:1.69-buster as builder

# WORKDIR /app


# # Copy the source code
# COPY . .

# # Build the application
# RUN cargo build --release


# # Production stage
# FROM debian:buster-slim

# WORKDIR /usr/local/bin

# COPY --from=builder /app/target/release/chatterfluxapi .

# EXPOSE 5001

# CMD ["./chatterfluxapi"]



# Start from a Rust image so we have Rust and cargo available
FROM rust:latest

# Install system dependencies
RUN apt-get update -y && \
    apt-get install -y clang libclang-dev make g++ lld liblz4-dev && \
    rm -rf /var/lib/apt/lists/* 

# Create a new empty shell project
WORKDIR /usr/src/chatterfluxapi

# Copy the Cargo.toml and Cargo.lock files to leverage Docker caching
COPY Cargo.toml Cargo.lock .env ./

# # This is a dummy build to get the dependencies cached
# RUN mkdir -p ./src && \
#     echo 'fn main() { println!("Dummy build"); }' > ./src/main.rs && \
#     cargo build --release

# Now copy your actual source code
COPY ./src ./src

# Build the application
RUN cargo build --release

EXPOSE 5001

CMD ["./target/release/chatterfluxapi"]
