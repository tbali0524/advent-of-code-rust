# Advent of Code solutions in Rust by TBali

![rust v1.91](https://shields.io/badge/rust-1.91-blue?logo=rust)
![build](https://img.shields.io/github/actions/workflow/status/tbali0524/advent-of-code-rust/qa.yml)
![AoC stars](https://img.shields.io/badge/total%20AoC%20⭐-240-green)
![license](https://img.shields.io/github/license/tbali0524/advent-of-code-rust)

* [AoC website](https://adventofcode.com/)
* My AoC username: `tbali0524`
* [Puzzle list](puzzles.md) with topics and my completion status (in Rust)
* [Results and running times](results.md)
* [This repo on GitHub](https://github.com/tbali0524/advent-of-code-rust)
* [My AoC solutions in PHP](https://github.com/tbali0524/advent-of-code-solutions) (complete 10 seasons, 506⭐)

This repo contains my AoC solutions in Rust, and a simple CLI runner. The first 9 seasons I originally solved in PHP, and later rewrote some solutions (~3.5 seasons) in Rust. From season 2024 I solved the puzzles first in Rust (and rewrote them in PHP later).

## Usage

```sh
# -- setup
# install Rust: https://www.rust-lang.org/tools/install
rustup update stable
# -- info
cargo version
cargo tree
# -- lint
cargo audit # needs cargo plugin: cargo-audit
cargo check
cargo fmt
cargo clippy
# -- doc
cargo doc --no-deps --document-private-items --open
# -- test
cargo nextest run # needs cargo plugin: <https://nexte.st/>
cargo test
cargo test 2025
cargo test 2025day01
cargo test cli
# in Powershell:
$Env:RUST_BACKTRACE=1; cargo test
cargo run
cargo run -- 2025
cargo run -- 2025 1
# -- run
cargo build --release
target/release/aoc.exe
target/release/aoc.exe 2025
target/release/aoc.exe 2025 1
cargo run --release
# -- shortcut run (Windows)
./aoc.bat
./aoc.bat 2025
./aoc.bat 2025 1
./aoc.bat --help
# -- shortcut qa+run (Windows)
./qa.ps1
# -- profiling (Windows), using [samply](https://github.com/mstange/samply/)
cargo build --profile profiling
samply record ./target/profiling/aoc.exe
# -- cleanup
cargo clean
```

## Adding a new solution

* for puzzle year `YYYY`, day `DD`:
* add puzzle input in `input/YYYY/AocYYYYDayDD.txt` and example inputs in `...exX.txt`
* add and edit source in `src/aocYYYY/aocYYYYdayDD.rs`, using the template in `src/aocYYYYdayDD.rs`
    * update `pub fn metadata()`, write `solve()`, add unit tests as needed
* edit `src/aocYYYY.rs`:
    * uncomment the `pub mod aocYYYYdayDD;` line
    * update the `PUZZLES` list: replace `None` with `Some(...)`
* for a completely new season:
    * edit `src/lib.rs`: add a `pub mod aocYYYY;` line
    * edit `src/aoc.rs`: increase `MAX_SEASONS` and add a `Some(...)` item to `PUZZLES`
    * add and update `src/aocYYYY.rs` using the template in `src/aocYYYY.rs`
