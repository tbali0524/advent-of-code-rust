//! [aoc](https://adventofcode.com/2024/day/0)

use crate::aoc::{PuzzleMetaData, PuzzleError, PuzzleInput, PuzzleResult};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2024,
        day: 0,
        title: "",
        solution: ("0", "0"),
        example_solutions: vec![("0", "0")],
    }
}

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Check input
    if input.len() != 1 {
        return Err(PuzzleError("Input must have a single line".into()));
    }
    // ---------- Part 1
    let mut ans1 = 0;
    // ---------- Part 2
    let mut ans2 = 0;
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
    fn example2() {
        test_case(metadata, solve, 2);
    }

    #[test]
    fn puzzle() {
        test_case(metadata, solve, 0);
    }

    #[test]
    #[ignore]
    fn invalid_single_line() {
        test_invalid(&[String::from("a"), String::from("b")], solve);
    }

    #[test]
    #[ignore]
    fn invalid() {
        test_invalid(&[String::from("a")], solve);
    }
}
