//! # Advent of Code solutions CLI runner binary crate

use aoc::aoc::cli::run;
use std::process::ExitCode;

pub fn main() -> ExitCode {
    run()
}
