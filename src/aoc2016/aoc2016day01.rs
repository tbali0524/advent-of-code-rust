//! [aoc](https://adventofcode.com/2016/day/1)

use crate::aoc::{PuzzleError, PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::collections::HashSet;

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2016,
        day: 1,
        title: "No Time for a Taxicab",
        solution: ("262", "131"),
        example_solutions: vec![("8", "4")],
    }
}

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    if input.len() != 1 {
        return Err(PuzzleError("input must have a single line".into()));
    }
    let instructions = input[0]
        .split(", ")
        .map(|x| {
            if x.len() < 2 {
                Err(PuzzleError("instruction must be at least 2 digits".into()))
            } else if !['R', 'L'].contains(&x.chars().next().unwrap()) {
                Err(PuzzleError("instruction must start with R or L".into()))
            } else {
                let moves_result = x[1..].parse::<i32>();
                if let Ok(moves) = moves_result {
                    Ok((if x.starts_with('R') { 1i32 } else { -1 }, moves))
                } else {
                    Err(PuzzleError(
                        "instruction must contain number of moves as integer".into(),
                    ))
                }
            }
        })
        .collect::<Result<Vec<_>, _>>()?;
    // ---------- Part 1
    const DELTAS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut x = 0;
    let mut y = 0;
    let mut direction = 0; // N
    for (turn, moves) in &instructions {
        direction = (direction + turn + 4) % 4;
        let (dx, dy) = DELTAS[direction as usize];
        x += dx * moves;
        y += dy * moves;
    }
    let ans1 = x.abs() + y.abs();
    // ---------- Part 2
    let mut ans2 = 0;
    let mut x = 0;
    let mut y = 0;
    let mut direction = 0; // N
    let mut memo = HashSet::new();
    memo.insert((x, y));
    'outer: for (turn, moves) in &instructions {
        direction = (direction + turn + 4) % 4;
        let (dx, dy) = DELTAS[direction as usize];
        for _ in 0..*moves as usize {
            x += dx;
            y += dy;
            if memo.contains(&(x, y)) {
                ans2 = x.abs() + y.abs();
                break 'outer;
            }
            memo.insert((x, y));
        }
    }
    Ok((ans1.to_string(), ans2.to_string()))
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
        test_invalid(&[&"R8", &"R4"], solve);
    }
}
