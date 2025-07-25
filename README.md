# Grey Hack Zero-day Countdown

A utility that will alert a Discord channel via the webhook environment variable when the next zero-day will occur.

## Prerequisites

A host with the following

* rust
* cargo
* rustup*

\* recommended

Tested on

* rust 1.88.0
* cargo 1.88.0
* rustup 1.88.0


## Setup

1. Copy the `.env.example` file as `.env`
1. Add a Discord channel webhook endpoint as the `WEBHOOK` variable in `.env`
1. Run `cargo build --release`

## How to run

Simply call the binary after setting the environment variables from `.env`

```bash
source ./.env && cargo run --release
```

You can also call the binary directly (on non-Windows, remove the `.exe` from the next command)

```bash
source ./.env && ./target/release/gh_zday_countdown.exe
```
