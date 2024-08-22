//! [aoc](https://adventofcode.com/2017/day/12)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::collections::{HashMap, HashSet};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2017,
        day: 12,
        title: "Digital Plumber",
        solution: ("288", "211"),
        example_solutions: vec![("6", "2")],
    }
}

type ItemType = usize;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let mut adj_list = HashMap::new();
    for line in input {
        let mut a = line.split(" <-> ");
        let node = a
            .next()
            .unwrap()
            .parse::<ItemType>()
            .map_err(|_| "input lines must start with an integer")?;
        let list = a
            .next()
            .ok_or("input lines must contain an <-> arrow")?
            .split(", ")
            .map(|x| {
                x.parse::<ItemType>().map_err(|_| {
                    format!("adjacence list must contain only integers, found `{}`", x)
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        adj_list.insert(node, list);
    }
    // ---------- Part 1 + 2
    let mut ans1 = 0;
    let mut ans2 = 0;
    let mut visited = HashSet::new();
    for from in 0..adj_list.len() {
        if visited.contains(&from) {
            continue;
        }
        ans2 += 1;
        let mut q = vec![from];
        visited.insert(from);
        let mut read_idx = 0;
        while read_idx < q.len() {
            if from == 0 {
                ans1 += 1;
            }
            let id = q[read_idx];
            read_idx += 1;
            for &nb in adj_list.get(&id).unwrap() {
                if visited.contains(&nb) {
                    continue;
                }
                visited.insert(nb);
                q.push(nb);
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
    fn invalid_lines_must_have_a_single_arrow() {
        test_invalid(&[&"0 <-> 1, 2", &"1"], solve);
    }

    #[test]
    fn invalid_right_of_arrow_only_contains_int() {
        test_invalid(&[&"0 <-> 1, 2", &"1 <-> 2, a"], solve);
    }

    #[test]
    fn invalid_left_of_arrow_only_single_int() {
        test_invalid(&[&"a <-> 1, 2"], solve);
    }
}
