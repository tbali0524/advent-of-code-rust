//! General definitions and a CLI runner.

pub mod runner;

pub const START_SEASON: usize = 2015;
pub const MAX_SEASONS: usize = 10;
pub const MAX_DAYS: usize = 25;

pub type PuzzleInput = Result<Vec<String>, &'static str>;
pub type PuzzleSolution = (String, String);
pub type PuzzleResult = Result<PuzzleSolution, &'static str>;
pub type Solver = fn(&[String]) -> PuzzleResult;
pub type Runner = fn() -> bool;
pub type Season = [Option<Runner>; MAX_DAYS];

/// Each solution must have a constant containing its metadata with this type.
pub struct PuzzleMetaData<'a> {
    pub year: usize,
    pub day: usize,
    pub title: &'a str,
    pub solution: (i64, i64),
    pub example_solutions: [(i64, i64); 2],
    pub string_solution: Option<(&'a str, &'a str)>, // use only for non-integer solutions
    pub example_string_solutions: Option<[(&'a str, &'a str); 2]>, // use only for non-integer solutions
    pub example_string_inputs: Option<[&'a str; 2]>, // use only for short, single-line example inputs
}

/// array of seasons that have implemented solutions
pub const PUZZLES: [Option<Season>; MAX_SEASONS] = [
    Some(crate::aoc2015::PUZZLES),
    Some(crate::aoc2016::PUZZLES),
    Some(crate::aoc2017::PUZZLES),
    Some(crate::aoc2018::PUZZLES),
    Some(crate::aoc2019::PUZZLES),
    Some(crate::aoc2020::PUZZLES),
    Some(crate::aoc2021::PUZZLES),
    Some(crate::aoc2022::PUZZLES),
    Some(crate::aoc2023::PUZZLES),
    None, // Some(crate::aoc2024::PUZZLES),
];
