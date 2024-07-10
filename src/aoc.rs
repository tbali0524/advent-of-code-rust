pub mod runner;

pub type Runner = fn() -> bool;

pub fn get_puzzles() -> Vec<(usize, Vec<Runner>)> {
    vec![
        // crate::aoc2015::get_puzzles(),
        // crate::aoc2016::get_puzzles(),
        crate::aoc2017::get_puzzles(),
        // crate::aoc2018::get_puzzles(),
        // crate::aoc2019::get_puzzles(),
        // crate::aoc2020::get_puzzles(),
        // crate::aoc2021::get_puzzles(),
        // crate::aoc2022::get_puzzles(),
        // crate::aoc2023::get_puzzles(),
    ]
}
