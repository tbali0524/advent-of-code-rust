//! [aoc](https://adventofcode.com/2017/day/1)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2017,
        day: 1,
        title: "Inverse Captcha",
        solution: ("1102", "1076"),
        example_solutions: vec![("9", "0"), ("0", "4")],
    }
}

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Check input
    if input.len() != 1 {
        Err("input must have a single line")?;
    }
    let line = &input[0];
    for c in line.chars() {
        if !c.is_ascii_digit() {
            Err(format!("input must contain only digits, found `{c}"))?;
        }
    }
    // ---------- Part 1
    let mut ans1 = 0;
    for i in 0..line.len() {
        let c = line.chars().nth(i).unwrap();
        if c == line.chars().nth((i + 1) % line.len()).unwrap() {
            ans1 += c.to_digit(10).unwrap();
        }
    }
    // ---------- Part 2
    let mut ans2 = 0;
    for i in 0..line.len() {
        let c = line.chars().nth(i).unwrap();
        if c == line.chars().nth((i + line.len() / 2) % line.len()).unwrap() {
            ans2 += c.to_digit(10).unwrap();
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
    fn example2() {
        test_case(metadata, solve, 2);
    }

    #[test]
    fn puzzle() {
        test_case(metadata, solve, 0);
    }

    #[test]
    fn invalid_single_line() {
        test_invalid_msg(&["123", "1"], solve, "input must have a single line");
    }

    #[test]
    fn invalid_only_digits() {
        test_invalid_msg(&["1a3"], solve, "input must contain only digits");
    }
}
