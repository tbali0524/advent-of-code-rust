# Advent of Code solutions in Rust by TBali

![rust v1.81](https://shields.io/badge/rust-1.81-blue?logo=rust)
![build](https://img.shields.io/github/actions/workflow/status/tbali0524/advent-of-code-rust/qa.yml)
![AoC stars](https://img.shields.io/badge/total%20AoC%20⭐-138-yellow)
![license](https://img.shields.io/github/license/tbali0524/advent-of-code-rust)

* [AoC website](https://adventofcode.com/)
* My AoC username: `tbali0524`
* [Puzzle list](puzzles.md) with topics and my completion status (in Rust)
* Link to this repo on [GitHub](https://github.com/tbali0524/advent-of-code-rust)

This repo contains the _partial_ rewrite of my complete (__450__ ⭐) [AoC solutions in PHP](https://github.com/tbali0524/advent-of-code-solutions) to `Rust`, plus a simple CLI runner.

## Usage

```sh
# -- setup
# install Rust: https://www.rust-lang.org/tools/install
rustup update stable
cargo version
# -- lint
cargo fmt
cargo clippy
# -- doc
cargo doc --no-deps --open
# -- test
cargo test
cargo test 2023
cargo test 2023day02
cargo test cli
# in Powershell:
$env:RUST_BACKTRACE=1 ; cargo test
cargo run
cargo run -- 2023
cargo run -- 2023 2
# -- run
cargo build --release
target/release/aoc.exe
target/release/aoc.exe 2023
target/release/aoc.exe 2023 2
# -- shortcut run
./aoc.bat
./aoc.bat 2023
./aoc.bat 2023 2
./aoc.bat --help
# -- shortcut qa
./qa.bat
# -- cleanup
cargo clean
```

## Adding a new solution

* add puzzle input in `input/year/AocXXXXDayXX.txt` and example inputs in `...exX.txt`
* add and edit source in `src/year/aocXXXXdayXX.rs`, using the template in `src/2024/aoc2024day00.rs`
    * update `pub fn metadata()`, write `solve()`, add unit tests as needed
* edit `src/aocXXXX.rs`:
    * uncomment the `pub mod aocXXXXdayXX;` line
    * update the `PUZZLES` list: replace `None` with `Some(...)`
* for a completely new season:
    * edit `src/lib.rs`: add a `pub mod aocXXXX;` line
    * edit `src/aoc.rs`: increase `MAX_SEASONS` and add a `Some(...)` item to `PUZZLES`
    * add and update `src/aocXXXX.rs` using the template in `src/aoc2024.rs`
