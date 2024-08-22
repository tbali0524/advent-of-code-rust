//! [aoc](https://adventofcode.com/2017/day/8)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};
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
            Err("invalid input: line must contain 7 items")?;
        }
        let reg = a[0];
        let sign = match a[1] {
            "inc" => 1,
            "dec" => -1,
            _ => Err(format!("invalid operator `{}`", a[1]))?,
        };
        let by = a[2]
            .parse::<ItemType>()
            .map_err(|_| format!("'by' operand must be an integer, found `{}`", a[2]))?;

        let operand1_reg = a[4];
        let operand1 = *regs.entry(operand1_reg).or_default();
        let comparison = a[5];
        let operand2 = a[6].parse::<ItemType>().map_err(|_| {
            format!(
                "second comparison operand must be an integer, found `{}`",
                a[6]
            )
        })?;
        let result = match comparison {
            "<" => operand1 < operand2,
            ">" => operand1 > operand2,
            "<=" => operand1 <= operand2,
            ">=" => operand1 >= operand2,
            "==" => operand1 == operand2,
            "!=" => operand1 != operand2,
            _ => Err(format!("invalid comparison operator `{}`", comparison))?,
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
        test_invalid(&[&"b inc 5 if a >"], solve);
    }

    #[test]
    fn invalid_operator() {
        test_invalid(&[&"b pow 5 if a > 1"], solve);
    }

    #[test]
    fn invalid_by_operand_must_be_integer() {
        test_invalid(&[&"b inc a if a > 1"], solve);
    }

    #[test]
    fn invalid_by_second_comparison_operand_must_be_integer() {
        test_invalid(&[&"b inc a if a > c"], solve);
    }

    #[test]
    fn invalid_by_comparison_operator() {
        test_invalid(&[&"b inc a if a ?= 1"], solve);
    }
}
