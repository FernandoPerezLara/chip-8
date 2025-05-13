##########################################
# Base
##########################################
FROM rust:bullseye AS base

# Create app directory
WORKDIR /usr/src/app


##########################################
# Build
##########################################
FROM base AS build

# Install system dependencies
RUN apt update && apt install -y libc6

# Install proyect dependencies
RUN rustup component add clippy rustfmt
RUN cargo install wasm-pack

COPY Cargo.toml Cargo.lock ./
