//! [aoc](https://adventofcode.com/2015/day/1)

use crate::aoc::{PuzzleError, PuzzleInput, PuzzleMetaData, PuzzleResult};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2015,
        day: 1,
        title: "Not Quite Lisp",
        solution: ("74", "1795"),
        example_solutions: vec![("-3", "1"), ("0", "5")],
    }
}

type ItemType = i32;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Check input
    if input.len() != 1 {
        return Err(PuzzleError("input must have a single line".into()));
    }
    let line = &input[0];
    for c in line.chars() {
        if c != '(' && c != ')' {
            return Err(PuzzleError("input must contain only ( or ) chars".into()));
        }
    }
    // ---------- Part 1
    let ans1 = line.matches('(').collect::<Vec<_>>().len() as ItemType
        - line.matches(')').collect::<Vec<_>>().len() as ItemType;
    // ---------- Part 2
    let mut ans2 = 0;
    let mut floor = 0;
    while ans2 < line.len() && floor >= 0 {
        let c = line.chars().nth(ans2).unwrap();
        if c == '(' {
            floor += 1;
        } else if c == ')' {
            floor -= 1;
        }
        ans2 += 1;
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
    fn example2() {
        test_case(metadata, solve, 2);
    }

    #[test]
    fn puzzle() {
        test_case(metadata, solve, 0);
    }

    #[test]
    fn invalid_single_line() {
        test_invalid(&[&"(())", &"()"], solve);
    }

    #[test]
    fn invalid_only_parentheses() {
        test_invalid(&[&"(a)"], solve);
    }
}
