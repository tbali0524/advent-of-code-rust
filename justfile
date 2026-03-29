# Config for [Just](https://just.systems/) command runner.

set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

# list all available recipes
default:
    @just --list

# check source (no change applied)
lint:
    cargo check
    cargo fmt --check
    cargo clippy

# open the generated docs
docs:
    cargo doc --no-deps --document-private-items --open

# run tests
test:
    cargo test
    @# cargo nextest run

# run all puzzles
run:
    cargo run --release

# run all QA checks: lint, test, run, docs
qa: lint test run docs
