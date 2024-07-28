//! Solutions for season 2015

pub mod aoc2015day01;
pub mod aoc2015day02;
pub mod aoc2015day03;
pub mod aoc2015day04;
pub mod aoc2015day05;
pub mod aoc2015day06;
pub mod aoc2015day07;
pub mod aoc2015day08;
pub mod aoc2015day09;
pub mod aoc2015day10;
pub mod aoc2015day11;
// pub mod aoc2015day12;
pub mod aoc2015day13;
// pub mod aoc2015day14;
// pub mod aoc2015day15;
// pub mod aoc2015day16;
pub mod aoc2015day17;
pub mod aoc2015day18;
// pub mod aoc2015day19;
// pub mod aoc2015day20;
// pub mod aoc2015day21;
// pub mod aoc2015day22;
// pub mod aoc2015day23;
// pub mod aoc2015day24;
pub mod aoc2015day25;

/// array of implemented solutions for the season
pub const PUZZLES: crate::aoc::Season = [
    Some((aoc2015day01::metadata, aoc2015day01::solve)),
    Some((aoc2015day02::metadata, aoc2015day02::solve)),
    Some((aoc2015day03::metadata, aoc2015day03::solve)),
    Some((aoc2015day04::metadata, aoc2015day04::solve)),
    Some((aoc2015day05::metadata, aoc2015day05::solve)),
    Some((aoc2015day06::metadata, aoc2015day06::solve)),
    Some((aoc2015day07::metadata, aoc2015day07::solve)),
    Some((aoc2015day08::metadata, aoc2015day08::solve)),
    Some((aoc2015day09::metadata, aoc2015day09::solve)),
    Some((aoc2015day10::metadata, aoc2015day10::solve)),
    Some((aoc2015day11::metadata, aoc2015day11::solve)),
    None, // Some((aoc2015day12::metadata, aoc2015day12::solve)),
    Some((aoc2015day13::metadata, aoc2015day13::solve)),
    None, // Some((aoc2015day14::metadata, aoc2015day14::solve)),
    None, // Some((aoc2015day15::metadata, aoc2015day15::solve)),
    None, // Some((aoc2015day16::metadata, aoc2015day16::solve)),
    Some((aoc2015day17::metadata, aoc2015day17::solve)),
    Some((aoc2015day18::metadata, aoc2015day18::solve)),
    None, // Some((aoc2015day19::metadata, aoc2015day19::solve)),
    None, // Some((aoc2015day20::metadata, aoc2015day20::solve)),
    None, // Some((aoc2015day21::metadata, aoc2015day21::solve)),
    None, // Some((aoc2015day22::metadata, aoc2015day22::solve)),
    None, // Some((aoc2015day23::metadata, aoc2015day23::solve)),
    None, // Some((aoc2015day24::metadata, aoc2015day24::solve)),
    Some((aoc2015day25::metadata, aoc2015day25::solve)),
];
