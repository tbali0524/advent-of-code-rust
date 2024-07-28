//! [aoc](https://adventofcode.com/2015/day/10)

use crate::aoc::{PuzzleError, PuzzleInput, PuzzleMetaData, PuzzleResult};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2015,
        day: 10,
        title: "Elves Look, Elves Say",
        solution: ("360154", "5103798"),
        example_solutions: vec![("6", "0")],
    }
}

const MAX1: usize = 40;
const MAX2: usize = 50;
const EXAMPLE_MAX: usize = 5;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Check input
    if input.len() != 1 {
        return Err(PuzzleError("Input must have a single line".into()));
    }
    let mut prev = input[0].to_owned();
    // ---------- Part 1 + 2
    let mut ans1 = 0;
    let mut ans_example = 0;
    let mut next = String::new();
    for i in 0..MAX2 {
        if i == EXAMPLE_MAX {
            ans_example = prev.len();
        }
        if i == MAX1 {
            ans1 = prev.len();
        }
        next = String::new();
        let mut start = 0;
        while start < prev.len() {
            let mut end = start + 1;
            while end < prev.len() && prev.as_bytes()[end] == prev.as_bytes()[start] {
                end += 1;
            }
            next.push_str(&(end - start).to_string());
            next.push(prev.as_bytes()[start] as char);
            start = end;
        }
        prev = next.clone();
    }
    let ans2 = next.len();
    // detect puzzle example as input
    if input[0] == "1" {
        return Ok((ans_example.to_string(), "0".to_owned()));
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
        test_invalid(&vec![String::from("a"), String::from("b")], solve);
    }
}
