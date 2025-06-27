//! [aoc](https://adventofcode.com/2024/day/19)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};
use core::cmp::Reverse;
use std::cmp::min;
use std::collections::{BinaryHeap, HashMap, HashSet};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2024,
        day: 19,
        title: "Linen Layout",
        solution: ("247", "692596560138745"),
        example_solutions: vec![("6", "16")],
    }
}

type ItemType = i64;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    if input.len() < 3 || !input[1].is_empty() {
        Err("patterns and designs must be separated by an empty line")?;
    }
    let patterns = input[0].split(", ").collect::<Vec<_>>();
    let max_pattern = patterns.iter().map(|&x| x.len()).max().unwrap();
    let designs = &input[2..];
    // ---------- Part 1 + 2
    let mut lookup = HashSet::new();
    for &pattern in &patterns {
        lookup.insert(pattern.to_owned());
    }
    let mut ans1 = 0;
    let mut ans2 = 0;
    for &design in designs {
        let mut found_sol = false;
        let mut has_partial = HashMap::<usize, ItemType>::new();
        let mut q = BinaryHeap::new();
        has_partial.insert(0, 1);
        q.push(Reverse(0));
        while let Some(Reverse(pos)) = q.pop() {
            let prev_sols = *has_partial.get(&pos).unwrap_or(&1);
            if pos == design.len() {
                if !found_sol {
                    ans1 += 1;
                    found_sol = true;
                }
                ans2 += *has_partial.get(&pos).unwrap_or(&1);
                continue;
            }
            let max_to_pos = min(design.len(), pos + max_pattern);
            for to_pos in (pos + 1)..=max_to_pos {
                if !lookup.contains(&design[pos..to_pos]) {
                    continue;
                }
                if !has_partial.contains_key(&to_pos) {
                    q.push(Reverse(to_pos));
                }
                has_partial
                    .entry(to_pos)
                    .and_modify(|counter| *counter += prev_sols)
                    .or_insert(prev_sols);
            }
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
    fn invalid_single_line() {
        test_invalid_msg(
            &["a, b", "a", "ab"],
            solve,
            "patterns and designs must be separated by an empty line",
        );
    }
}
