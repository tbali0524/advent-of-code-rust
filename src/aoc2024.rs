//! Solutions for season 2024

pub mod aoc2024day01;
pub mod aoc2024day02;
pub mod aoc2024day03;
pub mod aoc2024day04;
pub mod aoc2024day05;
pub mod aoc2024day06;
pub mod aoc2024day07;
pub mod aoc2024day08;
pub mod aoc2024day09;
pub mod aoc2024day10;
pub mod aoc2024day11;
pub mod aoc2024day12;
pub mod aoc2024day13;
pub mod aoc2024day14;
pub mod aoc2024day15;
pub mod aoc2024day16;
pub mod aoc2024day17;
pub mod aoc2024day18;
pub mod aoc2024day19;
pub mod aoc2024day20;
pub mod aoc2024day21;
pub mod aoc2024day22;
pub mod aoc2024day23;
pub mod aoc2024day24;
pub mod aoc2024day25;

/// array of implemented solutions for the season
pub const PUZZLES: crate::aoc::Season = [
    Some((aoc2024day01::metadata, aoc2024day01::solve)),
    Some((aoc2024day02::metadata, aoc2024day02::solve)),
    Some((aoc2024day03::metadata, aoc2024day03::solve)),
    Some((aoc2024day04::metadata, aoc2024day04::solve)),
    Some((aoc2024day05::metadata, aoc2024day05::solve)),
    Some((aoc2024day06::metadata, aoc2024day06::solve)),
    Some((aoc2024day07::metadata, aoc2024day07::solve)),
    Some((aoc2024day08::metadata, aoc2024day08::solve)),
    Some((aoc2024day09::metadata, aoc2024day09::solve)),
    Some((aoc2024day10::metadata, aoc2024day10::solve)),
    Some((aoc2024day11::metadata, aoc2024day11::solve)),
    Some((aoc2024day12::metadata, aoc2024day12::solve)),
    Some((aoc2024day13::metadata, aoc2024day13::solve)),
    Some((aoc2024day14::metadata, aoc2024day14::solve)),
    Some((aoc2024day15::metadata, aoc2024day15::solve)),
    Some((aoc2024day16::metadata, aoc2024day16::solve)),
    Some((aoc2024day17::metadata, aoc2024day17::solve)),
    Some((aoc2024day18::metadata, aoc2024day18::solve)),
    Some((aoc2024day19::metadata, aoc2024day19::solve)),
    Some((aoc2024day20::metadata, aoc2024day20::solve)),
    Some((aoc2024day21::metadata, aoc2024day21::solve)),
    Some((aoc2024day22::metadata, aoc2024day22::solve)),
    Some((aoc2024day23::metadata, aoc2024day23::solve)),
    Some((aoc2024day24::metadata, aoc2024day24::solve)),
    Some((aoc2024day25::metadata, aoc2024day25::solve)),
];
