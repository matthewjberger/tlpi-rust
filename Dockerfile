FROM rust:latest as builder
WORKDIR /usr/src/tlpi-rust
RUN apt-get update && apt-get upgrade