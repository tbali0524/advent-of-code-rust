// This is module aoc2017: solutions for a season

pub mod aoc2017day01;
pub mod aoc2017day02;
// pub mod aoc2017day04;
// pub mod aoc2017day05;
// pub mod aoc2017day06;
// pub mod aoc2017day07;
// pub mod aoc2017day08;
// pub mod aoc2017day09;
// pub mod aoc2017day10;
// pub mod aoc2017day11;
// pub mod aoc2017day12;
// pub mod aoc2017day13;
// pub mod aoc2017day14;
// pub mod aoc2017day15;
// pub mod aoc2017day16;
// pub mod aoc2017day17;
// pub mod aoc2017day18;
// pub mod aoc2017day19;
// pub mod aoc2017day20;
// pub mod aoc2017day21;
// pub mod aoc2017day22;
// pub mod aoc2017day23;
// pub mod aoc2017day24;
// pub mod aoc2017day25;

use crate::aoc::Runner;

pub fn get_puzzles() -> (usize, Vec<Runner>) {
    (2017, vec![
        crate::aoc2017::aoc2017day01::run,
        crate::aoc2017::aoc2017day02::run,
    ])
}
