# Config file for [Just](https://just.systems/)

set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

lint:
    cargo check
    cargo fmt --check
    cargo clippy

docs:
    cargo doc --no-deps --document-private-items --open

test:
    cargo test
    # cargo nextest run

run:
    cargo run --release
