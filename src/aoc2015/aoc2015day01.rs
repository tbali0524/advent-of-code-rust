//! [aoc](https://adventofcode.com/2015/day/1)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};

pub const PUZZLE_METADATA: PuzzleMetaData<'static> = PuzzleMetaData {
    year: 2015,
    day: 1,
    title: "Not Quite Lisp",
    solution: (74, 1795),
    example_solutions: [(-3, 1), (0, 5)],
    string_solution: None,
    example_string_solutions: None,
    example_string_inputs: Some([")())())", "()())"]),
};

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Check input
    if input.len() != 1 {
        return Err("Input must have a single line");
    }
    let line = &input[0];
    for c in line.chars() {
        if c != '(' && c != ')' {
            return Err("Input must contain only ( or ) chars");
        }
    }
    // ---------- Part 1
    let ans1 = line.matches('(').collect::<Vec<_>>().len() as i64
        - line.matches(')').collect::<Vec<_>>().len() as i64;
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
        test_case(&PUZZLE_METADATA, 1, solve);
    }

    #[test]
    fn example2() {
        test_case(&PUZZLE_METADATA, 2, solve);
    }

    #[test]
    fn puzzle() {
        test_case(&PUZZLE_METADATA, 0, solve);
    }

    #[test]
    fn invalid_single_line() {
        test_invalid(
            &PUZZLE_METADATA,
            &[String::from("(())"), String::from("()")],
            solve,
        );
    }

    #[test]
    fn invalid_only_parentheses() {
        test_invalid(&PUZZLE_METADATA, &[String::from("(a)")], solve);
    }
}
