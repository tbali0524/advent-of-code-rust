# Advent of Code solutions in Rust by TBali

![rust v1.79](https://shields.io/badge/rust-1.79-blue?logo=rust)
![build](https://img.shields.io/github/actions/workflow/status/tbali0524/advent-of-code-rust/qa.yml)
![AoC stars](https://img.shields.io/badge/total%20AoC%20⭐-60-yellow)
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
cargo install
# -- lint
cargo fmt
cargo clippy
# -- doc
cargo doc --open
# -- test
cargo test
cargo test 2017
cargo test 2017day02
cargo test runner
cargo run
cargo run -- 2017
cargo run -- 2017 2
# -- run
cargo build --release
target\release\advent-of-code-rust.exe
target\release\advent-of-code-rust.exe 2017
target\release\advent-of-code-rust.exe 2017 2
# -- shortcut run
.\aoc.bat
.\aoc.bat 2017
.\aoc.bat 2017 2
.\aoc.bat --help
# -- shortcut qa
.\qa.bat
# -- cleanup
cargo clean
```

## Adding a new solution

* add puzzle input in `input/year/AocXXXXDayXX.txt` and example inputs `...exX.txt`
* add and edit source in `src/year/aocXXXXdayXX.rs`, use the template in `src/2024/aoc2024day00.rs`
    * update `PUZZLE_METADATA`, write `solve()`, add unit tests as needed
* uncomment the `pub mod aocXXXXdayXX;` line in `src/aocXXXX.rs`,
* update the `PUZZLES` list in `src/aocXXXX.rs`, replacing `None` with `Some(...)`
* for a completely new season:
    * uncomment the `mod aocXXXX;` line in `src/main.rs`
    * update the `PUZZLES` list in `src/aoc.rs`, replacing `None` with `Some(...)`
    * add and update `src/aocXXXX.rs` based on template in `src/aoc2024.rs`
