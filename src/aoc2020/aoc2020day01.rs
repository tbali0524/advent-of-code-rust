//! [aoc](https://adventofcode.com/2020/day/1)

use crate::aoc::PuzzleMetaData;
use crate::aoc::PuzzleResult;
use std::collections::HashMap;
use std::collections::HashSet;

pub const PUZZLE_METADATA: PuzzleMetaData<'static> = PuzzleMetaData {
    year: 2020,
    day: 1,
    title: "Report Repair",
    solution: (988771, 171933104),
    example_solutions: [(514579, 241861950), (0, 0)],
    string_solution: None,
    example_string_solutions: None,
    example_string_inputs: None,
};

type ItemType = i32;

pub fn solve(input: &[String]) -> PuzzleResult {
    // ---------- Parse and Check input
    let data = input
        .iter()
        .map(|line| {
            line.parse::<ItemType>()
                .map_err(|_| "Input must contain only integers")
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
    fn puzzle() {
        test_case(&PUZZLE_METADATA, 0, solve);
    }

    #[test]
    fn invalid_only_contains_int() {
        test_invalid(&PUZZLE_METADATA, &[String::from("a")], solve);
    }
}
