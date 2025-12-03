//! [aoc](https://adventofcode.com/2025/day/3)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2025,
        day: 3,
        title: "Lobby",
        solution: ("17179", "170025781683941"),
        example_solutions: vec![("357", "3121910778619")],
    }
}

type ItemType = u64;

const COUNT_DIGITS_PART2: usize = 12;

#[allow(clippy::needless_range_loop)]
pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let mut banks = Vec::new();
    for &line in input.iter() {
        let bank = line
            .chars()
            .map(|c| c.to_digit(10).ok_or("input must contain only digits"))
            .collect::<Result<Vec<u32>, _>>()?;
        banks.push(bank);
    }
    // ---------- Part 1
    let mut ans1 = 0;
    for bank in banks.iter() {
        let mut pos_digit1 = 0;
        let mut digit1 = 0;
        for pos in 0..bank.len().saturating_sub(1) {
            if bank[pos] > digit1 {
                digit1 = bank[pos];
                pos_digit1 = pos;
            }
        }
        let digit2 = bank[(pos_digit1 + 1)..].iter().max().unwrap_or(&0);
        ans1 += digit1 * 10 + digit2;
    }
    // ---------- Part 2
    let mut ans2 = 0;
    for bank in banks.iter() {
        let mut best = 0u64;
        let mut prev_pos_digit = 0;
        for i in 0..COUNT_DIGITS_PART2 {
            let mut pos_digit = prev_pos_digit;
            let mut digit = 0;
            for pos in prev_pos_digit..bank.len().saturating_sub(COUNT_DIGITS_PART2 - 1 - i) {
                if bank[pos] > digit {
                    digit = bank[pos];
                    pos_digit = pos;
                }
            }
            prev_pos_digit = pos_digit + 1;
            best = 10 * best + digit as ItemType;
        }
        ans2 += best;
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
    fn invalid_must_contain_only_digits() {
        test_invalid_msg(&[&"1a"], solve, "input must contain only digits");
    }
}
