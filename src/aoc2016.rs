//! Solutions for season 2016

pub mod aoc2016day01;
pub mod aoc2016day02;
pub mod aoc2016day03;
pub mod aoc2016day04;
pub mod aoc2016day05;
pub mod aoc2016day06;
pub mod aoc2016day07;
pub mod aoc2016day08;
pub mod aoc2016day09;
// pub mod aoc2016day10;
// pub mod aoc2016day11;
// pub mod aoc2016day12;
// pub mod aoc2016day13;
// pub mod aoc2016day14;
// pub mod aoc2016day15;
// pub mod aoc2016day16;
// pub mod aoc2016day17;
// pub mod aoc2016day18;
pub mod aoc2016day19;
// pub mod aoc2016day20;
// pub mod aoc2016day21;
// pub mod aoc2016day22;
// pub mod aoc2016day23;
// pub mod aoc2016day24;
// pub mod aoc2016day25;

/// array of implemented solutions for the season
pub const PUZZLES: crate::aoc::Season = [
    Some((aoc2016day01::metadata, aoc2016day01::solve)),
    Some((aoc2016day02::metadata, aoc2016day02::solve)),
    Some((aoc2016day03::metadata, aoc2016day03::solve)),
    Some((aoc2016day04::metadata, aoc2016day04::solve)),
    Some((aoc2016day05::metadata, aoc2016day05::solve)),
    Some((aoc2016day06::metadata, aoc2016day06::solve)),
    Some((aoc2016day07::metadata, aoc2016day07::solve)),
    Some((aoc2016day08::metadata, aoc2016day08::solve)),
    Some((aoc2016day09::metadata, aoc2016day09::solve)),
    None, // Some((aoc2016day10::metadata, aoc2016day10::solve)),
    None, // Some((aoc2016day11::metadata, aoc2016day11::solve)),
    None, // Some((aoc2016day12::metadata, aoc2016day12::solve)),
    None, // Some((aoc2016day13::metadata, aoc2016day13::solve)),
    None, // Some((aoc2016day14::metadata, aoc2016day14::solve)),
    None, // Some((aoc2016day15::metadata, aoc2016day15::solve)),
    None, // Some((aoc2016day16::metadata, aoc2016day16::solve)),
    None, // Some((aoc2016day17::metadata, aoc2016day17::solve)),
    None, // Some((aoc2016day18::metadata, aoc2016day18::solve)),
    Some((aoc2016day19::metadata, aoc2016day19::solve)),
    None, // Some((aoc2016day20::metadata, aoc2016day20::solve)),
    None, // Some((aoc2016day21::metadata, aoc2016day21::solve)),
    None, // Some((aoc2016day22::metadata, aoc2016day22::solve)),
    None, // Some((aoc2016day23::metadata, aoc2016day23::solve)),
    None, // Some((aoc2016day24::metadata, aoc2016day24::solve)),
    None, // Some((aoc2016day25::metadata, aoc2016day25::solve)),
];
