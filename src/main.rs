// Advent of Code - CLI runner

mod aoc;
mod aoc2015;
// mod aoc2016;
mod aoc2017;
// mod aoc2018;
// mod aoc2019;
// mod aoc2020;
// mod aoc2021;
// mod aoc2022;
// mod aoc2023;

use std::process::ExitCode;
use crate::aoc::runner::*;

pub fn main() -> ExitCode {
    println!("{}\n", crate::aoc::runner::MSG_TITLE);
    match crate::aoc::runner::parse_args() {
        Err(msg) => {
            crate::aoc::runner::print_help();
            println!("{ANSI_RED}[ERROR]{ANSI_RESET} {}\n", msg);
            ExitCode::from(2)
        }
        Ok((year, day)) => {
            let result = crate::aoc::runner::run_puzzles(year, day);
            if result { ExitCode::SUCCESS } else { ExitCode::from(1) }
        }
    }
}
