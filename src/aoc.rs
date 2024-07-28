//! Advent of Code general types, runner and the CLI.

use std::error;
use std::fmt;

pub mod ansi;
pub mod cli;
pub mod runner;

pub const START_SEASON: usize = 2015;
pub const MAX_SEASONS: usize = 10;
pub const MAX_DAYS: usize = 25;

#[derive(PartialEq)]
pub struct PuzzleError(pub String);
impl error::Error for PuzzleError {}
impl fmt::Debug for PuzzleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Input Error: {}", self.0)
    }
}
impl fmt::Display for PuzzleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Input Error: {}", self.0)
    }
}

pub type ReadInputResult = Result<Vec<String>, PuzzleError>;
pub type PuzzleInput<'a> = &'a Vec<String>;
pub type PuzzleExpected<'a> = (&'a str, &'a str);
pub type PuzzleSolution = (String, String);
pub type PuzzleResult = Result<PuzzleSolution, PuzzleError>;
pub type MetaData<'a> = fn() -> PuzzleMetaData<'a>;
pub type Solver = fn(PuzzleInput) -> PuzzleResult;
pub type Puzzle<'a> = (MetaData<'a>, Solver);
pub type Season<'a> = [Option<Puzzle<'a>>; MAX_DAYS];

/// Each solution must have a constant containing its metadata with this type.
pub struct PuzzleMetaData<'a> {
    pub year: usize,
    pub day: usize,
    pub title: &'a str,
    pub solution: PuzzleExpected<'a>,
    pub example_solutions: Vec<PuzzleExpected<'a>>,
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
