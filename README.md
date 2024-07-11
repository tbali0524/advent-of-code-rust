# Advent of Code solutions in Rust by TBali

![rust v1.79](https://shields.io/badge/rust-1.79-blue?logo=rust)
![AoC stars](https://img.shields.io/badge/total%20AoC%20⭐-8-yellow)
![license](https://img.shields.io/github/license/tbali0524/advent-of-code-rust)

* [AoC website](https://adventofcode.com/)
* My AoC username: `tbali0524`
* [Puzzle list](puzzles.md) with topics and my completion status
* Link to this repo on [GitHub](https://github.com/tbali0524/advent-of-code-rust)

This repo contains the rewrite of my [AoC solutions in PHP](https://github.com/tbali0524/advent-of-code-solutions) to Rust, plus a simple CLI runner.

## Usage

```sh
# install rust: https://www.rust-lang.org/tools/install
rustup update stable
cargo version
cargo install
cargo test
cargo test 2017
cargo test 2017day02
cargo clippy
cargo run
cargo run -- 2017
cargo run -- 2017 2
cargo build --release
.\aoc.bat
.\aoc.bat 2017
.\aoc.bat 2017 2
target\release\advent-of-code-rust.exe
target\release\advent-of-code-rust.exe 2017
target\release\advent-of-code-rust.exe 2017 2
cargo clean
```

## Adding new solution

* test inputs shall go to `input/year/AocXXXXDayXX.txt` and `...exX.txt`
* source shall go to `src/year/aocXXXXdayXX.rs`, use the template in `src/2024/aoc2024day00.rs`
* uncomment the mod line and the item in the `PUZZLES` list in `src/aocXXXX.rs`
* for a new season: uncomment the mod line and update the `PUZZLES` list in `src/aoc.rs`
