// https://adventofcode.com/2015/day/3

use crate::aoc::PuzzleMetaData;
use crate::aoc::PuzzleResult;
use std::collections::HashSet;

pub const PUZZLE_METADATA: PuzzleMetaData<'static> = PuzzleMetaData {
    year: 2015,
    day: 3,
    title: "Perfectly Spherical Houses in a Vacuum",
    solutions: (2592, 2360),
    example_solutions: [(4, 3), (2, 11)],
    example_string_inputs: ["^>v<", "^v^v^v^v^v"],
};

pub fn delta(dir: char) -> Result<(i32, i32), &'static str> {
    match dir {
        '>' => Ok((1, 0)),
        'v' => Ok((0, 1)),
        '<' => Ok((-1, 0)),
        '^' => Ok((0, -1)),
        _ => Err("Invalid direction"),
    }
}

pub fn solve(input: &[String]) -> PuzzleResult {
    // ---------- Check input
    if input.len() != 1 {
        return Err("Input must have a single line");
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
            &[String::from("<>"), String::from("><")],
            solve,
        );
    }

    #[test]
    fn invalid_only_directions() {
        test_invalid(&PUZZLE_METADATA, &[String::from("<a>")], solve);
    }
}
