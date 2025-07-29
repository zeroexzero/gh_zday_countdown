FROM rust:1.88-alpine3.22

# Necessary build files
RUN apk add --no-cache build-base

# Create build artifact project
RUN USER=root cargo new --bin build-artifact
WORKDIR /build-artifact
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm -rf src/*

# Environment setup
WORKDIR /opt/gh_zday_countdown
ENV WEBHOOK=REPLACE_ME
ENV ALERT_THRESHOLD=86400
ENV IRL_GH_ORIGIN=1739039409

# Copy necessary runtime & source files
COPY Cargo.lock ./Cargo.lock
COPY Cargo.toml ./Cargo.toml
COPY src ./src

# RUN cargo install --path .
RUN cargo build --release

CMD ["./target/release/gh_zday_countdown"]
