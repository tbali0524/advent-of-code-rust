//! [aoc](https://adventofcode.com/2020/day/1)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::collections::{HashMap, HashSet};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2020,
        day: 1,
        title: "Report Repair",
        solution: ("988771", "171933104"),
        example_solutions: vec![("514579", "241861950")],
    }
}

type ItemType = i32;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let data = input
        .iter()
        .map(|&line| {
            line.parse::<ItemType>()
                .map_err(|_| format!("input must contain only integers, found `{}`", line))
        })
        .collect::<Result<Vec<_>, _>>()?;
    // ---------- Part 1
    let mut ans1 = 0;
    let mut visited = HashSet::new();
    for item in &data {
        if visited.contains(&(2020 - item)) {
            ans1 = item * (2020 - item);
            break;
        }
        visited.insert(item);
    }
    // ---------- Part 2
    let mut ans2 = 0;
    let mut memo = HashMap::new();
    for (idx1, item1) in data.iter().enumerate() {
        for (idx2, item2) in data.iter().enumerate() {
            if idx1 != idx2 {
                memo.insert(item1 + item2, item1 * item2);
            }
        }
    }
    for item in &data {
        if memo.contains_key(&(2020 - item)) {
            ans2 = item * memo.get(&(2020 - item)).unwrap();
            break;
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
