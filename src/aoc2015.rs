//! Solutions for season 2015

pub mod aoc2015day01;
pub mod aoc2015day02;
pub mod aoc2015day03;
pub mod aoc2015day04;
// pub mod aoc2015day05;
// pub mod aoc2015day06;
// pub mod aoc2015day07;
// pub mod aoc2015day08;
// pub mod aoc2015day09;
// pub mod aoc2015day10;
// pub mod aoc2015day11;
// pub mod aoc2015day12;
// pub mod aoc2015day13;
// pub mod aoc2015day14;
// pub mod aoc2015day15;
// pub mod aoc2015day16;
// pub mod aoc2015day17;
// pub mod aoc2015day18;
// pub mod aoc2015day19;
// pub mod aoc2015day20;
// pub mod aoc2015day21;
// pub mod aoc2015day22;
// pub mod aoc2015day23;
// pub mod aoc2015day24;
pub mod aoc2015day25;

/// array of implemented solutions for the season
pub const PUZZLES: crate::aoc::Season = [
    Some((aoc2015day01::PUZZLE_METADATA, aoc2015day01::solve)),
    Some((aoc2015day02::PUZZLE_METADATA, aoc2015day02::solve)),
    Some((aoc2015day03::PUZZLE_METADATA, aoc2015day03::solve)),
    Some((aoc2015day04::PUZZLE_METADATA, aoc2015day04::solve)),
    None, // Some((aoc2015day05::PUZZLE_METADATA, aoc2015day05::solve)),
    None, // Some((aoc2015day06::PUZZLE_METADATA, aoc2015day06::solve)),
    None, // Some((aoc2015day07::PUZZLE_METADATA, aoc2015day07::solve)),
    None, // Some((aoc2015day08::PUZZLE_METADATA, aoc2015day08::solve)),
    None, // Some((aoc2015day09::PUZZLE_METADATA, aoc2015day09::solve)),
    None, // Some((aoc2015day10::PUZZLE_METADATA, aoc2015day10::solve)),
    None, // Some((aoc2015day11::PUZZLE_METADATA, aoc2015day11::solve)),
    None, // Some((aoc2015day12::PUZZLE_METADATA, aoc2015day12::solve)),
    None, // Some((aoc2015day13::PUZZLE_METADATA, aoc2015day13::solve)),
    None, // Some((aoc2015day14::PUZZLE_METADATA, aoc2015day14::solve)),
    None, // Some((aoc2015day15::PUZZLE_METADATA, aoc2015day15::solve)),
    None, // Some((aoc2015day16::PUZZLE_METADATA, aoc2015day16::solve)),
    None, // Some((aoc2015day17::PUZZLE_METADATA, aoc2015day17::solve)),
    None, // Some((aoc2015day18::PUZZLE_METADATA, aoc2015day18::solve)),
    None, // Some((aoc2015day19::PUZZLE_METADATA, aoc2015day19::solve)),
    None, // Some((aoc2015day20::PUZZLE_METADATA, aoc2015day20::solve)),
    None, // Some((aoc2015day21::PUZZLE_METADATA, aoc2015day21::solve)),
    None, // Some((aoc2015day22::PUZZLE_METADATA, aoc2015day22::solve)),
    None, // Some((aoc2015day23::PUZZLE_METADATA, aoc2015day23::solve)),
    None, // Some((aoc2015day24::PUZZLE_METADATA, aoc2015day24::solve)),
    Some((aoc2015day25::PUZZLE_METADATA, aoc2015day25::solve)),
];
