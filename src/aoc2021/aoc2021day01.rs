//! [aoc](https://adventofcode.com/2021/day/1)

use crate::aoc::{PuzzleError, PuzzleInput, PuzzleMetaData, PuzzleResult};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2021,
        day: 1,
        title: "Sonar Sweep",
        solution: ("1477", "1523"),
        example_solutions: vec![("7", "5")],
    }
}

type ItemType = i32;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let data = input
        .iter()
        .map(|line| {
            line.parse::<ItemType>()
                .map_err(|_| PuzzleError("input must contain only integers".into()))
        })
        .collect::<Result<Vec<_>, _>>()?;
    // ---------- Part 1
    let mut ans1 = 0;
    for i in 1..data.len() {
        if data[i] > data[i - 1] {
            ans1 += 1;
        }
    }
    // ---------- Part 2
    let mut ans2 = 0;
    const WINDOW: usize = 3;
    for i in WINDOW..data.len() {
        if data[i] > data[i - WINDOW] {
            ans2 += 1;
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
    fn invalid_only_contains_int() {
        test_invalid(&[&"a"], solve);
    }
}
