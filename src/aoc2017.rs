//! Solutions for season 2017

pub mod aoc2017day01;
pub mod aoc2017day02;
pub mod aoc2017day03;
pub mod aoc2017day04;
pub mod aoc2017day05;
pub mod aoc2017day06;
pub mod aoc2017day07;
pub mod aoc2017day08;
pub mod aoc2017day09;
pub mod aoc2017day10;
pub mod aoc2017day11;
pub mod aoc2017day12;
pub mod aoc2017day13;
pub mod aoc2017day14;
pub mod aoc2017day15;
pub mod aoc2017day16;
pub mod aoc2017day17;
pub mod aoc2017day18;
pub mod aoc2017day19;
pub mod aoc2017day20;
pub mod aoc2017day21;
pub mod aoc2017day22;
pub mod aoc2017day23;
pub mod aoc2017day24;
pub mod aoc2017day25;

/// array of implemented solutions for the season
pub const PUZZLES: crate::aoc::Season = [
    Some((aoc2017day01::metadata, aoc2017day01::solve)),
    Some((aoc2017day02::metadata, aoc2017day02::solve)),
    Some((aoc2017day03::metadata, aoc2017day03::solve)),
    Some((aoc2017day04::metadata, aoc2017day04::solve)),
    Some((aoc2017day05::metadata, aoc2017day05::solve)),
    Some((aoc2017day06::metadata, aoc2017day06::solve)),
    Some((aoc2017day07::metadata, aoc2017day07::solve)),
    Some((aoc2017day08::metadata, aoc2017day08::solve)),
    Some((aoc2017day09::metadata, aoc2017day09::solve)),
    Some((aoc2017day10::metadata, aoc2017day10::solve)),
    Some((aoc2017day11::metadata, aoc2017day11::solve)),
    Some((aoc2017day12::metadata, aoc2017day12::solve)),
    Some((aoc2017day13::metadata, aoc2017day13::solve)),
    Some((aoc2017day14::metadata, aoc2017day14::solve)),
    Some((aoc2017day15::metadata, aoc2017day15::solve)),
    Some((aoc2017day16::metadata, aoc2017day16::solve)),
    Some((aoc2017day17::metadata, aoc2017day17::solve)),
    Some((aoc2017day18::metadata, aoc2017day18::solve)),
    Some((aoc2017day19::metadata, aoc2017day19::solve)),
    Some((aoc2017day20::metadata, aoc2017day20::solve)),
    Some((aoc2017day21::metadata, aoc2017day21::solve)),
    Some((aoc2017day22::metadata, aoc2017day22::solve)),
    Some((aoc2017day23::metadata, aoc2017day23::solve)),
    Some((aoc2017day24::metadata, aoc2017day24::solve)),
    Some((aoc2017day25::metadata, aoc2017day25::solve)),
];
