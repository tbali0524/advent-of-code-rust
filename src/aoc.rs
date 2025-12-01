//! Advent of Code common type definitions, and submodules with runner and CLI functions.

pub mod ansi;
pub mod cli;
pub mod error;
pub mod runner;

pub const START_SEASON: usize = 2015;
pub const MAX_SEASONS: usize = 12; // empty YYYY season also included as a template
pub const MAX_DAYS: usize = 25;

// very slow puzzles (taking >5s) are excluded by default. They can be run individually.
pub const SKIP_SLOW: bool = true;
pub const PUZZLES_TO_SKIP: [(usize, usize); 3] = [(2016, 5), (2023, 23), (2024, 7)];
pub const DURATION_THRESHOLD_MILLIS: u64 = 1000; // puzzle duration printed in yellow if taking longer than this

pub use error::PuzzleError;

/// The expected solution for a test case, containing both parts of the puzzle.
pub type PuzzleExpected<'a> = (&'a str, &'a str);

/// The parameter type of `the solve()` functions: the puzzle input, already split to lines
pub type PuzzleInput<'a> = &'a [&'a str];

/// A candidate solution for a test case, containing both parts of the puzzle.
pub type PuzzleSolution = (String, String);

/// The return type of the `solve()` functions.
pub type PuzzleResult = Result<PuzzleSolution, PuzzleError>;

/// Each solution must have a `metadata()` function with this signature.
pub type MetaData<'a> = fn() -> PuzzleMetaData<'a>;

/// Each solution must have a `solve()` function with this signature.
pub type Solver = fn(PuzzleInput) -> PuzzleResult;

/// An implemented puzzle: its `metadata()` and `solve()` functions, used by the `PUZZLES` constants in all season modules.
pub type Puzzle<'a> = (MetaData<'a>, Solver);

/// The array of the implemented puzzles, used by the [`PUZZLES`] constant in this (`aoc`) module.
pub type Season<'a> = [Option<Puzzle<'a>>; MAX_DAYS];

/// Each solution must have a `metadata()` function, returning an instance of this struct.
pub struct PuzzleMetaData<'a> {
    pub year: usize,
    pub day: usize,
    pub title: &'a str,
    pub solution: PuzzleExpected<'a>,
    pub example_solutions: Vec<PuzzleExpected<'a>>,
}

/// Array of seasons containing the arrays of the implemented puzzles.
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
    Some(crate::aoc2024::PUZZLES),
    Some(crate::aoc2025::PUZZLES),
    None, // Some(crate::aocYYYY::PUZZLES),
];
