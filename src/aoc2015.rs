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
    Some(crate::aoc2015::aoc2015day01::run),
    Some(crate::aoc2015::aoc2015day02::run),
    Some(crate::aoc2015::aoc2015day03::run),
    Some(crate::aoc2015::aoc2015day04::run),
    None, // Some(crate::aoc2015::aoc2015day05::run),
    None, // Some(crate::aoc2015::aoc2015day06::run),
    None, // Some(crate::aoc2015::aoc2015day07::run),
    None, // Some(crate::aoc2015::aoc2015day08::run),
    None, // Some(crate::aoc2015::aoc2015day09::run),
    None, // Some(crate::aoc2015::aoc2015day10::run),
    None, // Some(crate::aoc2015::aoc2015day11::run),
    None, // Some(crate::aoc2015::aoc2015day12::run),
    None, // Some(crate::aoc2015::aoc2015day13::run),
    None, // Some(crate::aoc2015::aoc2015day14::run),
    None, // Some(crate::aoc2015::aoc2015day15::run),
    None, // Some(crate::aoc2015::aoc2015day16::run),
    None, // Some(crate::aoc2015::aoc2015day17::run),
    None, // Some(crate::aoc2015::aoc2015day18::run),
    None, // Some(crate::aoc2015::aoc2015day19::run),
    None, // Some(crate::aoc2015::aoc2015day20::run),
    None, // Some(crate::aoc2015::aoc2015day21::run),
    None, // Some(crate::aoc2015::aoc2015day22::run),
    None, // Some(crate::aoc2015::aoc2015day23::run),
    None, // Some(crate::aoc2015::aoc2015day24::run),
    Some(crate::aoc2015::aoc2015day25::run),
];
