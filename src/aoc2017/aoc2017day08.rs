//! [aoc](https://adventofcode.com/2017/day/8)

use crate::aoc::{PuzzleError, PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::collections::HashMap;

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2017,
        day: 8,
        title: "I Heard You Like Registers",
        solution: ("4567", "5636"),
        example_solutions: vec![("1", "10")],
    }
}

type ItemType = i32;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parsing and Checking input + Part 1 + 2
    let mut ans2 = 0;
    let mut regs = HashMap::new();
    for line in input {
        let a = line.split(' ').collect::<Vec<_>>();
        if a.len() != 7 {
            return Err(PuzzleError(
                "Invalid input: line must contain 7 items".into(),
            ));
        }
        let reg = a[0];
        let sign = match a[1] {
            "inc" => 1,
            "dec" => -1,
            _ => Err(PuzzleError("Invalid operator".into()))?,
        };
        let by = a[2]
            .parse::<ItemType>()
            .map_err(|_| PuzzleError("By operand must be an integer".into()))?;

        let operand1_reg = a[4];
        let operand1 = *regs.entry(operand1_reg).or_default();
        let comparison = a[5];
        let operand2 = a[6]
            .parse::<ItemType>()
            .map_err(|_| PuzzleError("Second comparison operand must be an integer".into()))?;
        let result = match comparison {
            "<" => operand1 < operand2,
            ">" => operand1 > operand2,
            "<=" => operand1 <= operand2,
            ">=" => operand1 >= operand2,
            "==" => operand1 == operand2,
            "!=" => operand1 != operand2,
            _ => Err(PuzzleError("Invalid comparison operator".into()))?,
        };
        if !result {
            continue;
        }
        let r = regs.entry(reg).or_default();
        *r += sign * by;
        if *r > ans2 {
            ans2 = *r;
        }
    }
    let ans1 = *regs.values().max().unwrap_or(&0);
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
    fn invalid_line_must_contain_7_items() {
        test_invalid(&vec![String::from("b inc 5 if a >")], solve);
    }

    #[test]
    fn invalid_operator() {
        test_invalid(&vec![String::from("b pow 5 if a > 1")], solve);
    }

    #[test]
    fn invalid_by_operand_must_be_integer() {
        test_invalid(&vec![String::from("b inc a if a > 1")], solve);
    }

    #[test]
    fn invalid_by_second_comparison_operand_must_be_integer() {
        test_invalid(&vec![String::from("b inc a if a > c")], solve);
    }

    #[test]
    fn invalid_by_comparison_operator() {
        test_invalid(&vec![String::from("b inc a if a ?= 1")], solve);
    }
}
