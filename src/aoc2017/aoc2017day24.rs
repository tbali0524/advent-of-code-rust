//! [aoc](https://adventofcode.com/2017/day/24)

use crate::aoc::{PuzzleError, PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::cmp;
use std::collections::{HashMap, HashSet};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2017,
        day: 24,
        title: "Electromagnetic Moat",
        solution: ("1656", "1642"),
        example_solutions: vec![("31", "19")],
    }
}

type ItemType = i64;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let components = input
        .iter()
        .map(|line| {
            line.split('/')
                .map(|x| {
                    x.parse::<ItemType>()
                        .map_err(|_| PuzzleError("Input must contain only integers".into()))
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
            if (used_bitmap & ((1 as ItemType) << idx)) != 0 {
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
    fn invalid_must_be_integers() {
        test_invalid(&[&"1/a"], solve);
    }
}
