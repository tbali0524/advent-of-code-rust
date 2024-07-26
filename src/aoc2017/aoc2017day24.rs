//! [aoc](https://adventofcode.com/2017/day/24)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::cmp;
use std::collections::{HashMap, HashSet};

pub const PUZZLE_METADATA: PuzzleMetaData<'static> = PuzzleMetaData {
    year: 2017,
    day: 24,
    title: "Electromagnetic Moat",
    solution: (1656, 1642),
    example_solutions: [(31, 19), (0, 0)],
    string_solution: None,
    example_string_solutions: None,
    example_string_inputs: None,
};

type ItemType = i64;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let components = input
        .iter()
        .map(|line| {
            line.split('/')
                .map(|x| {
                    x.parse::<ItemType>()
                        .map_err(|_| "Input must contain only integers")
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<Vec<_>>, _>>()?;
    // ---------- Part 1 + 2
    let mut max_per_length = HashMap::new();
    let mut visited = HashSet::from([(0, 0, 0, 0)]);
    let mut q = vec![(0, 0, 0, 0)];
    let mut read_idx = 0;
    while read_idx < q.len() {
        let (used_bitmap, right_port, strength, length) = q[read_idx];
        read_idx += 1;
        max_per_length
            .entry(length)
            .and_modify(|v| *v = cmp::max(strength, *v))
            .or_insert(strength);
        for (idx, component) in components.iter().enumerate() {
            if (used_bitmap & (1i64 << idx)) != 0 {
                continue;
            }
            for i in 0..2 {
                let left = component[i];
                if left != right_port {
                    continue;
                }
                let new_bitmap = used_bitmap | (1 << idx);
                let new_right = component[1 - i];
                let new_strength = strength + left + new_right;
                let new_length = length + 1;
                let hash = (new_bitmap, new_right, new_strength, new_length);
                if visited.contains(&hash) {
                    continue;
                }
                q.push(hash);
                visited.insert(hash);
            }
        }
    }
    let ans1 = *max_per_length.values().max().unwrap();
    let max_length = max_per_length.keys().max().unwrap();
    let ans2 = *max_per_length.get(max_length).unwrap();
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
    fn invalid_must_be_integers() {
        test_invalid(&PUZZLE_METADATA, &[String::from("1/a")], solve);
    }
}
