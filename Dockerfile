FROM rust:1.58 AS builder
WORKDIR /usr/src/app

RUN rustup target add x86_64-unknown-linux-musl

COPY . .

# Build and install radiologger-cli
RUN cargo build --package radiologger-cli --release && \
    cargo install --target x86_64-unknown-linux-musl --path ./radiologger-cli && \
    strip /usr/local/cargo/bin/radiologger

# Build and install target executable
ARG RL_PROJECT_NAME
RUN cargo build --package $RL_PROJECT_NAME --release && \
    cargo install --target x86_64-unknown-linux-musl --path ./$RL_PROJECT_NAME

FROM alpine:3.15
WORKDIR /usr/run/app

COPY --from=builder /usr/local/cargo/bin/radiologger .

ARG RL_EXECUTABLE_NAME
COPY --from=builder /usr/local/cargo/bin/$RL_EXECUTABLE_NAME .

ENV RL_EXECUTABLE_NAME=$RL_EXECUTABLE_NAME
USER 1000
CMD ./$RL_EXECUTABLE_NAME
