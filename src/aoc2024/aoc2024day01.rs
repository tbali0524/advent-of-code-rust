//! [aoc](https://adventofcode.com/2024/day/1)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2024,
        day: 1,
        title: "?",
        solution: ("0", "0"),
        example_solutions: vec![("0", "0")],
    }
}

pub fn solve(_input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    //
    // ---------- Part 1
    let ans1 = 0;
    // ---------- Part 2
    let ans2 = 0;
    Ok((ans1.to_string(), ans2.to_string()))
}

// ------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc::runner::tests::*;

    #[test]
    #[ignore]
    fn example1() {
        test_case(metadata, solve, 1);
    }

    #[test]
    #[ignore]
    fn example2() {
        test_case(metadata, solve, 2);
    }

    #[test]
    #[ignore]
    fn puzzle() {
        test_case(metadata, solve, 0);
    }

    #[test]
    #[ignore]
    fn invalid() {
        test_invalid_msg(&[&"a"], solve, "...");
    }
}
