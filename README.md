# Grey Hack Zero-day Countdown

A utility that will alert a Discord channel via the webhook environment variable when the next zero-day will occur.

## Prerequisites

If not running in Docker,  
A host with the following

* rust
* cargo
* rustup*

\* recommended

Tested on

* rust 1.88.0
* cargo 1.88.0
* rustup 1.88.0
* Docker 28.1.0

## Setup

1. Copy the `.env.example` file as `.env`
1. Add a Discord channel webhook endpoint as the `WEBHOOK` variable in `.env`
1. With the `ALERT_ENABLED` and `ALERT_THRESHOLD` you can tune the webhook alerting
1. Run `cargo build --release`

## How to run

See [Docker](#docker) or keep reading.

Note that you'll need to manually set the environment variables in the `.env` file unless using Docker.


```bash
cargo run --release
```

You can also call the binary directly (on non-Windows, remove the `.exe` from the next command)

```bash
./target/release/gh_zday_countdown.exe
```

## Docker

This app can also run inside docker to eliminate any host dependencies, including compilation.  
Yes, you don't even need rust-lang or cargo installed to actually run this project.

The benefits with using Docker is that the environment is taken care of, you don't need to set any environment variables as it's handled automatically.

```bash
docker compose run --rm gh_zday_countdown
```
