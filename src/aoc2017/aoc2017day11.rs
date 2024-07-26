//! [aoc](https://adventofcode.com/2017/day/11)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::cmp::max;

pub const PUZZLE_METADATA: PuzzleMetaData<'static> = PuzzleMetaData {
    year: 2017,
    day: 11,
    title: "Hex Ed",
    solution: (824, 1548),
    example_solutions: [(3, 3), (3, 3)],
    string_solution: None,
    example_string_solutions: None,
    example_string_inputs: Some(["ne,ne,ne", "se,sw,se,sw,sw"]),
};

pub fn cube_delta(dir: &str) -> Result<(i32, i32, i32), &'static str> {
    match dir {
        "n" => Ok((0, -1, 1)),
        "s" => Ok((0, 1, -1)),
        "nw" => Ok((-1, 0, 1)),
        "se" => Ok((1, 0, -1)),
        "ne" => Ok((1, -1, 0)),
        "sw" => Ok((-1, 1, 0)),
        _ => Err("Invalid direction"),
    }
}

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Check input
    if input.len() != 1 {
        return Err("Input must have a single line");
    }
    let data = input[0].split(',').collect::<Vec<_>>();
    // ---------- Part 1 + 2
    let mut ans1 = 0;
    let mut ans2 = 0;
    let mut q = 0;
    let mut r = 0;
    let mut s = 0;
    for dir in data {
        let (dq, dr, ds) = cube_delta(dir)?;
        (q, r, s) = (q + dq, r + dr, s + ds);
        ans1 = (q.abs() + r.abs() + s.abs()) / 2;
        ans2 = max(ans1, ans2);
    }
    Ok((ans1.to_string(), ans2.to_string()))
}

// ------------------------------------------------------------
// --- boilerplate below ---

pub fn run() -> bool {
    crate::aoc::runner::run_puzzle(&PUZZLE_METADATA, solve)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc::runner::tests::*;

    #[test]
    fn example1() {
        test_case(&PUZZLE_METADATA, 1, solve);
    }

    #[test]
    fn puzzle() {
        test_case(&PUZZLE_METADATA, 0, solve);
    }

    #[test]
    fn invalid_single_line() {
        test_invalid(
            &PUZZLE_METADATA,
            &[String::from("se,n"), String::from("s")],
            solve,
        );
    }

    #[test]
    fn invalid_direction() {
        test_invalid(&PUZZLE_METADATA, &[String::from("se,a,n")], solve);
    }
}
