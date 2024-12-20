//! [aoc](https://adventofcode.com/2024/day/7)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2024,
        day: 7,
        title: "Bridge Repair",
        solution: ("3351424677624", "204976636995111"),
        example_solutions: vec![("3749", "11387")],
    }
}

type ItemType = u64;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let mut results = Vec::new();
    let mut operands = Vec::new();
    for &row in input {
        let a = row.split_whitespace().collect::<Vec<&str>>();
        if a.len() < 3 {
            Err("missing operands")?;
        }
        if !a[0].ends_with(':') {
            Err("result must be followed by a `:`")?;
        }
        results.push(
            a[0][..(a[0].len() - 1)]
                .parse::<ItemType>()
                .map_err(|_| "result must be a non-negative integer")?,
        );
        operands.push(
            a[1..]
                .iter()
                .map(|&x| {
                    x.parse::<ItemType>().map_err(|_| {
                        format!("operands must be non-negative integers, found `{}`", x)
                    })
                })
                .collect::<Result<Vec<_>, _>>()?,
        );
    }
    // ---------- Part 1
    let mut ans1 = 0;
    for (idx, &result) in results.iter().enumerate() {
        let max = 1 << (operands[idx].len() - 1);
        for perm in 0..max {
            let mut calculation = operands[idx][0];
            for i in 0..(operands[idx].len() - 1) {
                calculation = match (perm >> i) & 1 {
                    0 => calculation * operands[idx][i + 1],
                    1 => calculation + operands[idx][i + 1],
                    _ => unreachable!(),
                };
                if calculation > result {
                    break;
                }
            }
            if calculation == result {
                ans1 += result;
                break;
            }
        }
    }
    // ---------- Part 2
    let mut ans2 = 0;
    for (idx, &result) in results.iter().enumerate() {
        let max = (3 as ItemType).pow((operands[idx].len() - 1) as u32);
        for perm in 0..max {
            let mut calculation = operands[idx][0];
            let mut remaining = perm;
            for i in 0..(operands[idx].len() - 1) {
                let operand = operands[idx][i + 1];
                calculation = match remaining % 3 {
                    0 => calculation * operand,
                    1 => calculation + operand,
                    2 => (calculation.to_string() + &operand.to_string())
                        .parse::<ItemType>()
                        .unwrap_or(result + 1),
                    _ => unreachable!(),
                };
                if calculation > result {
                    break;
                }
                remaining /= 3;
            }
            if calculation == result {
                ans2 += result;
                break;
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
    fn invalid_missing_colon() {
        test_invalid_msg(&[&"6 2 3"], solve, "result must be followed by a `:`");
    }

    #[test]
    fn invalid_result_must_be_integer() {
        test_invalid_msg(&[&"a: 2 3"], solve, "result must be a non-negative integer");
    }

    #[test]
    fn invalid_operand_must_be_integer() {
        test_invalid_msg(
            &[&"6: 2 a"],
            solve,
            "operands must be non-negative integers",
        );
    }

    #[test]
    fn invalid_needs_at_least_2_operands() {
        test_invalid_msg(&[&"6: 6"], solve, "missing operands");
    }
}
