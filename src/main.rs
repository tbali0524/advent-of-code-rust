// Advent of Code - CLI runner

mod aoc;
// mod aoc2015;
// mod aoc2016;
mod aoc2017;
// mod aoc2018;
// mod aoc2019;
// mod aoc2020;
// mod aoc2021;
// mod aoc2022;
// mod aoc2023;

use std::process::ExitCode;

pub fn main() -> ExitCode {
    let result = crate::aoc::runner::run_all();
    if result { ExitCode::SUCCESS } else { ExitCode::from(1) }
}
