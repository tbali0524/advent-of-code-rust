//! [aoc](https://adventofcode.com/2024/day/2)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::iter::zip;

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2024,
        day: 2,
        title: "Red-Nosed Reports",
        solution: ("572", "612"),
        example_solutions: vec![("2", "4")],
    }
}

type ItemType = i32;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let data = input
        .iter()
        .map(|&line| {
            line.split_whitespace()
                .map(|x| {
                    x.parse::<ItemType>()
                        .map_err(|_| format!("input must contain only integers, found `{}`", x))
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<Vec<_>>, _>>()?;

    // ---------- Part 1
    let ans1 = data.iter().filter(|&row| is_safe(row)).count();
    // ---------- Part 2
    let mut ans2 = 0;
    for row in data.iter() {
        if is_safe(row) {
            ans2 += 1;
            continue;
        }
        for i in 0..row.len() {
            let mut removed = row[..i].to_vec();
            removed.extend(&row[(i + 1)..]);
            if is_safe(&removed) {
                ans2 += 1;
                break;
            }
        }
    }
    Ok((ans1.to_string(), ans2.to_string()))
}

fn is_safe(row: &[i32]) -> bool {
    let is_inc = zip(row.iter().take(row.len() - 1), row.iter().skip(1))
        .all(|(&a, &b)| b - a >= 1 && b - a <= 3);
    let is_dec = zip(row.iter().take(row.len() - 1), row.iter().skip(1))
        .all(|(&a, &b)| a - b >= 1 && a - b <= 3);
    is_inc || is_dec
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
    fn invalid_must_have_contain_integers() {
        test_invalid_msg(&[&"a 2"], solve, "input must contain only integers");
    }
}
