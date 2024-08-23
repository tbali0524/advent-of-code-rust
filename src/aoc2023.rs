//! Solutions for season 2023

pub mod aoc2023day01;
pub mod aoc2023day02;
pub mod aoc2023day03;
pub mod aoc2023day04;
// pub mod aoc2023day05;
// pub mod aoc2023day06;
// pub mod aoc2023day07;
// pub mod aoc2023day08;
// pub mod aoc2023day09;
// pub mod aoc2023day10;
pub mod aoc2023day11;
// pub mod aoc2023day12;
// pub mod aoc2023day13;
// pub mod aoc2023day14;
// pub mod aoc2023day15;
// pub mod aoc2023day16;
// pub mod aoc2023day17;
// pub mod aoc2023day18;
// pub mod aoc2023day19;
// pub mod aoc2023day20;
// pub mod aoc2023day21;
// pub mod aoc2023day22;
// pub mod aoc2023day23;
// pub mod aoc2023day24;
// pub mod aoc2023day25;

/// array of implemented solutions for the season
pub const PUZZLES: crate::aoc::Season = [
    Some((aoc2023day01::metadata, aoc2023day01::solve)),
    Some((aoc2023day02::metadata, aoc2023day02::solve)),
    Some((aoc2023day03::metadata, aoc2023day03::solve)),
    Some((aoc2023day04::metadata, aoc2023day04::solve)),
    None, // Some((aoc2023day05::metadata, aoc2023day05::solve)),
    None, // Some((aoc2023day06::metadata, aoc2023day06::solve)),
    None, // Some((aoc2023day07::metadata, aoc2023day07::solve)),
    None, // Some((aoc2023day08::metadata, aoc2023day08::solve)),
    None, // Some((aoc2023day09::metadata, aoc2023day09::solve)),
    None, // Some((aoc2023day10::metadata, aoc2023day10::solve)),
    Some((aoc2023day11::metadata, aoc2023day11::solve)),
    None, // Some((aoc2023day12::metadata, aoc2023day12::solve)),
    None, // Some((aoc2023day13::metadata, aoc2023day13::solve)),
    None, // Some((aoc2023day14::metadata, aoc2023day14::solve)),
    None, // Some((aoc2023day15::metadata, aoc2023day15::solve)),
    None, // Some((aoc2023day16::metadata, aoc2023day16::solve)),
    None, // Some((aoc2023day17::metadata, aoc2023day17::solve)),
    None, // Some((aoc2023day18::metadata, aoc2023day18::solve)),
    None, // Some((aoc2023day19::metadata, aoc2023day19::solve)),
    None, // Some((aoc2023day20::metadata, aoc2023day20::solve)),
    None, // Some((aoc2023day21::metadata, aoc2023day21::solve)),
    None, // Some((aoc2023day22::metadata, aoc2023day22::solve)),
    None, // Some((aoc2023day23::metadata, aoc2023day23::solve)),
    None, // Some((aoc2023day24::metadata, aoc2023day24::solve)),
    None, // Some((aoc2023day25::metadata, aoc2023day25::solve)),
];
