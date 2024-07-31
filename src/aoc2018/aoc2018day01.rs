//! [aoc](https://adventofcode.com/2018/day/1)

use crate::aoc::{PuzzleError, PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::collections::HashSet;

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2018,
        day: 1,
        title: "Chronal Calibration",
        solution: ("590", "83445"),
        example_solutions: vec![("3", "2")],
    }
}

type ItemType = i32;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let data = input
        .iter()
        .map(|line| {
            line.parse::<ItemType>()
                .map_err(|_| PuzzleError("Input must contain only integers".into()))
        })
        .collect::<Result<Vec<_>, _>>()?;
    // ---------- Part 1
    let ans1 = data.iter().sum::<ItemType>();
    // ---------- Part 2
    let ans2;
    let mut memo = HashSet::new();
    let mut freq = 0;
    'outer: loop {
        for delta in &data {
            freq += delta;
            if memo.contains(&freq) {
                ans2 = freq;
                break 'outer;
            }
            memo.insert(freq);
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
