//! [aoc](https://adventofcode.com/2017/day/6)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::collections::HashMap;

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2017,
        day: 6,
        title: "Memory Reallocation",
        solution: ("7864", "1695"),
        example_solutions: vec![("5", "4")],
    }
}

type ItemType = i32;

#[expect(clippy::needless_range_loop)]
pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    if input.len() != 1 {
        Err("input must have a single line")?;
    }
    let data = input[0]
        .split_whitespace()
        .map(|x| {
            x.parse::<ItemType>()
                .map_err(|_| format!("input must contain only integers, found `{}`", x))
        })
        .collect::<Result<Vec<_>, _>>()?;
    // ---------- Part 1 + 2
    let ans1;
    let ans2;
    let n = data.len();
    let mut banks = data;
    let mut memo = HashMap::new();
    memo.insert(banks.clone(), 0);
    let mut step = 0;
    loop {
        let mut idx_max = 0;
        let mut max = ItemType::MIN;
        for idx_bank in 0..n {
            if banks[idx_bank] > max {
                idx_max = idx_bank;
                max = banks[idx_bank];
            }
        }
        let to_add = max / n as ItemType;
        let remainder = max % n as ItemType;
        banks[idx_max] = 0;
        for i in 1..=n {
            banks[(idx_max + i) % n] += to_add + if i as ItemType <= remainder { 1 } else { 0 };
        }
        step += 1;
        if memo.contains_key(&banks) {
            ans1 = step;
            ans2 = step - memo.get(&banks).unwrap();
            break;
        }
        memo.insert(banks.clone(), step);
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
        test_invalid(&[&"123", &"1"], solve);
    }

    #[test]
    fn invalid_only_contains_int() {
        test_invalid(&[&"1 a 3"], solve);
    }
}
