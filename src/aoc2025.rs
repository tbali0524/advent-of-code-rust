//! Solutions for season 2025

pub mod aoc2025day01;
pub mod aoc2025day02;
pub mod aoc2025day03;
// pub mod aoc2025day04;
// pub mod aoc2025day05;
// pub mod aoc2025day06;
// pub mod aoc2025day07;
// pub mod aoc2025day08;
// pub mod aoc2025day09;
// pub mod aoc2025day10;
// pub mod aoc2025day11;
// pub mod aoc2025day12;

/// array of implemented solutions for the season
pub const PUZZLES: crate::aoc::Season = [
    Some((aoc2025day01::metadata, aoc2025day01::solve)),
    Some((aoc2025day02::metadata, aoc2025day02::solve)),
    Some((aoc2025day03::metadata, aoc2025day03::solve)),
    None, // Some((aoc2025day04::metadata, aoc2025day04::solve)),
    None, // Some((aoc2025day05::metadata, aoc2025day05::solve)),
    None, // Some((aoc2025day06::metadata, aoc2025day06::solve)),
    None, // Some((aoc2025day07::metadata, aoc2025day07::solve)),
    None, // Some((aoc2025day08::metadata, aoc2025day08::solve)),
    None, // Some((aoc2025day09::metadata, aoc2025day09::solve)),
    None, // Some((aoc2025day10::metadata, aoc2025day10::solve)),
    None, // Some((aoc2025day11::metadata, aoc2025day11::solve)),
    None, // Some((aoc2025day12::metadata, aoc2025day12::solve)),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
];
