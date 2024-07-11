// module aoc: generic definitions and CLI runner

pub mod runner;

pub const START_SEASON: usize = 2015;
pub const MAX_SEASONS: usize = 10;
pub const MAX_DAYS: usize = 25;

pub type PuzzleInput = Result<Vec<String>, &'static str>;
pub type PuzzleResult = Result<(String, String), &'static str>;
pub type Solver = fn(&[String]) -> PuzzleResult;
pub type Runner = fn() -> bool;
pub type Season = [Option<Runner>; MAX_DAYS];

pub struct PuzzleMetaData<'a> {
    pub year: u32,
    pub day: u32,
    pub title: &'a str,
    pub solutions: (i64, i64),
    pub example_solutions: [(i64, i64); 2],
    pub example_string_inputs: [&'a str; 2],
}

pub const PUZZLES: [Option<crate::aoc::Season>; MAX_SEASONS] = [
    Some(crate::aoc2015::PUZZLES),
    None, // Some(crate::aoc2016::PUZZLES),
    Some(crate::aoc2017::PUZZLES),
    None, // Some(crate::aoc2018::PUZZLES),
    None, // Some(crate::aoc2019::PUZZLES),
    None, // Some(crate::aoc2020::PUZZLES),
    None, // Some(crate::aoc2021::PUZZLES),
    None, // Some(crate::aoc2022::PUZZLES),
    None, // Some(crate::aoc2023::PUZZLES),
    None, // Some(crate::aoc2024::PUZZLES),
];
