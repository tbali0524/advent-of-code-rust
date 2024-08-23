//! # Advent of Code solutions CLI runner binary crate.

#![deny(broken_intra_doc_links)]

use aoc::aoc::cli::run;
use std::process::ExitCode;

pub fn main() -> ExitCode {
    run()
}
