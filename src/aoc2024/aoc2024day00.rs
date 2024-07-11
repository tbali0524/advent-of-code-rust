// https://adventofcode.com/2024/day/0

use crate::aoc::PuzzleMetaData;
use crate::aoc::PuzzleResult;

pub const PUZZLE_METADATA: PuzzleMetaData<'static> = PuzzleMetaData {
    year: 2024,
    day: 0,
    title: "",
    solutions: (0, 0),
    example_solutions: [(0, 0), (0, 0)],
    example_string_inputs: ["", ""],
};

pub fn solve(input: &[String]) -> PuzzleResult {
    // ---------- Check input
    if input.len() != 1 {
        return Err("Input must have a single line");
    }
    // ---------- Part 1
    let mut ans1 = 0;
    // ---------- Part 2
    let mut ans2 = 0;
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
    #[ignore]
    fn invalid_single_line() {
        test_invalid(&PUZZLE_METADATA, &[String::from("a"), String::from("b")], solve);
    }

    #[test]
    #[ignore]
    fn invalid() {
        test_invalid(&PUZZLE_METADATA, &[String::from("a")], solve);
    }
}
