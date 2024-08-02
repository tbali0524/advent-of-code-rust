//! [aoc](https://adventofcode.com/2017/day/11)

use crate::aoc::{PuzzleError, PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::cmp::max;

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2017,
        day: 11,
        title: "Hex Ed",
        solution: ("824", "1548"),
        example_solutions: vec![("3", "3"), ("3", "3")],
    }
}

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Check input
    if input.len() != 1 {
        return Err(PuzzleError("input must have a single line".into()));
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

pub fn cube_delta(dir: &str) -> Result<(i32, i32, i32), PuzzleError> {
    match dir {
        "n" => Ok((0, -1, 1)),
        "s" => Ok((0, 1, -1)),
        "nw" => Ok((-1, 0, 1)),
        "se" => Ok((1, 0, -1)),
        "ne" => Ok((1, -1, 0)),
        "sw" => Ok((-1, 1, 0)),
        _ => Err(PuzzleError("invalid direction".into())),
    }
}

// ------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc::runner::tests::*;

    #[test]
    fn example1() {
        test_case(metadata, solve, 1);
    }

    #[test]
    fn puzzle() {
        test_case(metadata, solve, 0);
    }

    #[test]
    fn invalid_single_line() {
        test_invalid(&[&"se,n", &"s"], solve);
    }

    #[test]
    fn invalid_direction() {
        test_invalid(&[&"se,a,n"], solve);
    }
}
