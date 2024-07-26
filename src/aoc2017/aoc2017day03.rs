//! [aoc](https://adventofcode.com/2017/day/3)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::collections::HashMap;

pub const PUZZLE_METADATA: PuzzleMetaData<'static> = PuzzleMetaData {
    year: 2017,
    day: 3,
    title: "Spiral Memory",
    solution: (475, 279138),
    example_solutions: [(3, 23), (31, 1968)],
    string_solution: None,
    example_string_solutions: None,
    example_string_inputs: Some(["12", "1024"]),
};

type ItemType = i32;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    if input.len() != 1 {
        return Err("Input must have a single line");
    }
    let n = input[0]
        .parse::<ItemType>()
        .map_err(|_| "Input must be a single integer")?;
    // ---------- Part 1
    let mut r = 1;
    while (r + 2) * (r + 2) < n {
        r += 2;
    }
    let outer = n - r * r;
    let pos = outer % (r + 1);
    let d_tangential = r / 2 + 1;
    let d_radial = (d_tangential - pos).abs();
    let ans1 = if n <= 1 { 0 } else { d_tangential + d_radial };
    // ---------- Part 2
    let mut ans2 = 1;
    let mut x = 0;
    let mut y = 0;
    let mut dx = 1;
    let mut dy = 0;
    let mut memo = HashMap::new();
    memo.insert((y, x), ans2);
    while ans2 <= n {
        x += dx;
        y += dy;
        ans2 = 0;
        for ny in -1..=1 {
            for nx in -1..=1 {
                if nx == 0 && ny == 0 {
                    continue;
                }
                ans2 += memo.get(&(y + ny, x + nx)).unwrap_or(&0);
            }
        }
        memo.insert((y, x), ans2);
        match (dx, dy) {
            (1, 0) => {
                if !memo.contains_key(&(y - 1, x)) {
                    (dx, dy) = (0, -1);
                }
            }
            (0, -1) => {
                if !memo.contains_key(&(y, x - 1)) {
                    (dx, dy) = (-1, 0);
                }
            }
            (-1, 0) => {
                if !memo.contains_key(&(y + 1, x)) {
                    (dx, dy) = (0, 1);
                }
            }
            (0, 1) => {
                if !memo.contains_key(&(y, x + 1)) {
                    (dx, dy) = (1, 0);
                }
            }
            (_, _) => return Err("Impossible"),
        }
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
    fn example2() {
        test_case(&PUZZLE_METADATA, 2, solve);
    }

    #[test]
    fn puzzle() {
        test_case(&PUZZLE_METADATA, 0, solve);
    }

    #[test]
    fn invalid_single_line() {
        test_invalid(
            &PUZZLE_METADATA,
            &[String::from("123"), String::from("1")],
            solve,
        );
    }

    #[test]
    fn invalid_only_int() {
        test_invalid(&PUZZLE_METADATA, &[String::from("a")], solve);
    }
}
