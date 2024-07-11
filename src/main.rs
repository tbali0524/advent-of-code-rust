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
// mod aoc2024;

use crate::aoc::runner::*;
use std::env;
use std::process::ExitCode;

pub fn main() -> ExitCode {
    println!("{}\n", MSG_TITLE);
    let args = env::args().collect::<Vec<_>>();
    match parse_args(&args) {
        Err(msg) => {
            if msg == ARG_VERSION {
                return ExitCode::SUCCESS;
            }
            print_help();
            if msg == ARG_HELP {
                return ExitCode::SUCCESS;
            }
            println!("{ANSI_RED_INV}[ERROR]{ANSI_RESET} {}\n", msg);
            ExitCode::from(2)
        }
        Ok((year, day)) => {
            let result = run_puzzles(year, day);
            if result {
                ExitCode::SUCCESS
            } else {
                ExitCode::from(1)
            }
        }
    }
}
