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
        Err("input must have a single line")?;
    }
    let instructions = input[0]
        .split(", ")
        .map(|x| {
            if x.len() < 2 {
                Err("instruction must be at least 2 digits")?
            } else if !['R', 'L'].contains(&x.chars().next().unwrap()) {
                Err("instruction must start with `R` or `L`")?
            } else {
                let moves_result = x[1..].parse::<i32>();
                if let Ok(moves) = moves_result {
                    Ok((if x.starts_with('R') { 1i32 } else { -1 }, moves))
                } else {
                    Err("instruction must contain number of moves as integer")?
                }
            }
        })
        .collect::<Result<Vec<_>, PuzzleError>>()?;
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
        test_invalid_msg(&["R8, R4", "R4"], solve, "input must have a single line");
    }

    #[test]
    fn invalid_instructions_must_be_at_least_two_letters() {
        test_invalid_msg(&["R, R4"], solve, "instruction must be at least 2 digits");
    }

    #[test]
    fn invalid_instructions_must_start_with_r_or_l() {
        test_invalid_msg(&["R8, a4"], solve, "instruction must start with `R` or `L`");
    }

    #[test]
    fn invalid_instructions_must_end_with_integer() {
        test_invalid_msg(
            &["R8, La"],
            solve,
            "instruction must contain number of moves as integer",
        );
    }
}
