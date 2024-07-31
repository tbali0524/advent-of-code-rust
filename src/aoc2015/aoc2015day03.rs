//! [aoc](https://adventofcode.com/2015/day/3)

use crate::aoc::{PuzzleError, PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::collections::HashSet;

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2015,
        day: 3,
        title: "Perfectly Spherical Houses in a Vacuum",
        solution: ("2592", "2360"),
        example_solutions: vec![("4", "3"), ("2", "11")],
    }
}

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Check input
    if input.len() != 1 {
        return Err(PuzzleError("Input must have a single line".into()));
    }
    // ---------- Part 1
    let mut ans1 = 1;
    let mut memo = HashSet::new();
    let mut x = 0;
    let mut y = 0;
    memo.insert((x, y));
    for c in input[0].chars() {
        let (dx, dy) = delta(c)?;
        x += dx;
        y += dy;
        if !memo.contains(&(x, y)) {
            ans1 += 1;
        }
        memo.insert((x, y).to_owned());
    }
    // ---------- Part 2
    let mut ans2 = 1;
    let mut memo = HashSet::new();
    let mut x = [0; 2];
    let mut y = [0; 2];
    memo.insert((x[0], y[0]));
    for (idx, c) in input[0].chars().enumerate() {
        let (dx, dy) = delta(c)?;
        let parity = idx % 2;
        x[parity] += dx;
        y[parity] += dy;
        if !memo.contains(&(x[parity], y[parity])) {
            ans2 += 1;
        }
        memo.insert((x[parity], y[parity]).to_owned());
    }
    Ok((ans1.to_string(), ans2.to_string()))
}

pub fn delta(dir: char) -> Result<(i32, i32), PuzzleError> {
    match dir {
        '>' => Ok((1, 0)),
        'v' => Ok((0, 1)),
        '<' => Ok((-1, 0)),
        '^' => Ok((0, -1)),
        _ => Err(PuzzleError("Invalid direction".into())),
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
    fn example2() {
        test_case(metadata, solve, 2);
    }

    #[test]
    fn puzzle() {
        test_case(metadata, solve, 0);
    }

    #[test]
    fn invalid_single_line() {
        test_invalid(&[&"<>", &"><"], solve);
    }

    #[test]
    fn invalid_only_directions() {
        test_invalid(&[&"<a>"], solve);
    }
}
