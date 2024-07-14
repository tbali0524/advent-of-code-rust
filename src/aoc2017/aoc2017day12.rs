//! [aoc](https://adventofcode.com/2017/day/12)

use crate::aoc::PuzzleMetaData;
use crate::aoc::PuzzleResult;
use std::collections::HashMap;
use std::collections::HashSet;

pub const PUZZLE_METADATA: PuzzleMetaData<'static> = PuzzleMetaData {
    year: 2017,
    day: 12,
    title: "Digital Plumber",
    solution: (288, 211),
    example_solutions: [(6, 2), (0, 0)],
    string_solution: None,
    example_string_solutions: None,
    example_string_inputs: None,
};

type ItemType = usize;

pub fn solve(input: &[String]) -> PuzzleResult {
    // ---------- Parse and Check input
    let mut adj_list = HashMap::new();
    for line in input {
        let mut a = line.split(" <-> ");
        let node = a
            .next()
            .unwrap()
            .parse::<ItemType>()
            .map_err(|_| "Input lines must start with an integer")?;
        let list = a
            .next()
            .ok_or("Input lines must contain an <-> arrow")?
            .split(", ")
            .map(|x| {
                x.parse::<ItemType>()
                    .map_err(|_| "Adjacence list must contain only integers")
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
    fn invalid_lines_must_have_a_single_arrow() {
        test_invalid(
            &PUZZLE_METADATA,
            &[String::from("0 <-> 1, 2"), String::from("1")],
            solve,
        );
    }

    #[test]
    fn invalid_right_of_arrow_only_contains_int() {
        test_invalid(
            &PUZZLE_METADATA,
            &[String::from("0 <-> 1, 2"), String::from("1 <-> 2, a")],
            solve,
        );
    }

    #[test]
    fn invalid_left_of_arrow_only_single_int() {
        test_invalid(&PUZZLE_METADATA, &[String::from("a <-> 1, 2")], solve);
    }
}
